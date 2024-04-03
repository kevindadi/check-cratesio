use deadpool_postgres::tokio_postgres::error;
use deadpool_postgres::{tokio_postgres, Runtime};
use diesel::{prelude::*, serialize::Output, sql_types::Timestamp};
use diesel_demo::schema::crate_infos::dsl::crate_infos;
use diesel_demo::schema::crate_infos::result;
use diesel_demo::{
    crate_info::{CrateInfo, CrateResult},
    schema::{crates, crates_categories},
};
use dotenvy::dotenv;
use once_cell::sync::Lazy;
use std::error::Error;
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{env, path::Path};
use tokio::process::Command;
use tokio::sync::Semaphore;
use tokio::time::timeout;
use tracing::{debug, info, instrument, Level};
use tracing_subscriber::FmtSubscriber;

const WORKERS: usize = 16;
const ITERATIONS: usize = 1000;

// 定义一个全局的 Mutex<String>，使用 Lazy 来延迟初始化
static GLOBAL_DATA: Lazy<Mutex<String>> = Lazy::new(|| {
    let initial_data = "/home/kevin/RustPTA/toys/embedded/".to_string();
    Mutex::new(initial_data)
});

#[tokio::main]
async fn main() {
    dotenv().ok();

    // let file = File::create("app.log").expect("创建日志文件失败");
    // 无连接池
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    //let cfg = Config::from_env()?;

    let results = crates::table
        .select((crates::id, crates::name, crates::repository))
        .inner_join(crates_categories::table.on(crates::id.eq(crates_categories::crate_id)))
        .filter(crates_categories::category_id.eq(2813))
        .load::<(i32, String, Option<String>)>(&mut connection)
        .expect("Error loading crates and categories");

    // 设置Git 最大并发数为24
    let semaphore = Arc::new(Semaphore::new(16));

    let clone_path = String::from("/home/kevin/RustPTA/toys/embedded/");
    let mut handlers = Vec::new();
    let crate_info_vec = Arc::new(Mutex::new(Vec::<CrateInfo>::new()));
    for (id, name, url) in results {
        let c_name = name.clone();
        let c_url = url.clone();
        let c_path = clone_path.clone();
        let c_info = crate_info_vec.clone();
        let permit = semaphore.clone();
        let handler = tokio::task::spawn(async move {
            let _permit = permit.acquire().await.unwrap();
            let info = check_crate(id, c_path, c_name, c_url).await;
            c_info.lock().unwrap().push(info);
        });
        handlers.push(handler);
        // let crate_path = clone_path.clone() + &name.clone();
        // let mut crate_info = CrateInfo::new(id, name, crate_path.clone());
        // println!("Crate: {:?}", crate_path);
        // match url {
        //     Some(url) => {
        //         let _ = clone_crate(&url, &crate_path).await;
        //         let crate_result = exec_pta(&crate_path).await.unwrap();
        //         if crate_result.compile_pass {
        //             match crate_result.reason {
        //                 Some(reason) => {
        //                     crate_info.result = reason;
        //                 }
        //                 _ => {
        //                     crate_info.result = "No DeadLock".to_string();
        //                 }
        //             }
        //         } else {
        //             crate_info.result = "Compile Not Pass".to_string();
        //         }
        //         let _ = insert_crate_results(&mut connection, &crate_info);
        //         // crate_info_vec.push(crate_info);
        //     }
        //     _ => {}
        // }
    }
    let mut results = Vec::with_capacity(handlers.len());
    for handle in handlers {
        match handle.await {
            Ok(h) => results.push(h),
            _ => {}
        }
        // results.push(handle.await.unwrap());
    }
    let _ = crate_info_vec
        .lock()
        .unwrap()
        .iter()
        .map(|x| insert_crate_results(&mut connection, x));
}

#[instrument]
async fn check_crate(
    id: i32,
    clone_path: String,
    name: String,
    url: Option<String>,
    // config: &Config,
) -> CrateInfo {
    // let pool = config
    //     .pg
    //     .create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls)
    //     .unwrap();
    let crate_path = clone_path.clone() + &name.clone();
    let mut crate_info = CrateInfo::new(id, name, crate_path.clone());

    match url {
        Some(url) => {
            let _ = clone_crate(&url, &crate_path).await.unwrap();
            let e_dir = crate_path.to_string().clone();
            let crate_exec_handle = tokio::task::spawn_blocking(|| exec_pta(e_dir));
            let crates_result: Result<CrateResult, Box<dyn std::error::Error>> =
                match crate_exec_handle.await {
                    Ok(crate_result) => Ok(crate_result),
                    Err(e) => Err(Box::new(e) as Box<dyn Error>),
                };
            match crates_result {
                Ok(crate_result) => {
                    if crate_result.compile_pass {
                        match crate_result.reason {
                            Some(reason) => {
                                crate_info.result = reason;
                            }
                            _ => {
                                crate_info.result = "No DeadLock".to_string();
                            }
                        }
                    } else {
                        crate_info.result = "Compile Not Pass".to_string();
                    }
                }
                _ => {}
            }
            //let _ = insert_crate_results(connection, &crate_info);
            // crate_info_vec.push(crate_info);
        }
        _ => {
            crate_info.result = "No Git Repository".to_string();
        }
    }
    println!(
        "crate: {}'s result: {:?}",
        crate_info.name, crate_info.result
    );
    crate_info
}

fn insert_crate_results(conn: &mut PgConnection, results: &CrateInfo) -> QueryResult<usize> {
    diesel::insert_into(crate_infos)
        .values(results)
        .execute(conn)
}

#[instrument]
async fn clone_crate(url: &str, dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // check repository has already exist
    if std::fs::read_dir(dir).map_or(false, |mut entries| entries.next().is_some()) {
        debug!("Crate: {:?} already exist", url);
    } else {
        let c_url = url.to_string().clone();
        let c_dir = dir.to_string().clone();
        let task = tokio::spawn(async move {
            let _ = Command::new("git")
                .arg("clone")
                .arg(c_url)
                .arg(c_dir)
                .output()
                .await;
        });

        if let Err(_) = timeout(Duration::from_secs(20), task).await {
            return Ok(());
        }
    }
    Ok(())
}

fn exec_pta(dir: String) -> CrateResult {
    let output = std::process::Command::new("cargo")
        .arg("pta")
        .current_dir(dir.clone())
        .env(
            "LD_LIBRARY_PATH",
            "/home/kevin/.rustup/toolchains/nightly-2023-09-13-x86_64-unknown-linux-gnu/lib/",
        )
        .output()
        .unwrap();

    let mut info = CrateResult::new(false);

    if output.status.success() {
        let stderr = String::from_utf8(output.stderr).unwrap();
        if stderr.contains("Lock") {
            info.compile_pass = true;
            info.no_deadlock = false;
            info.reason = Some(stderr.clone());
        }
        info.no_deadlock = true;
    }

    let _ = std::process::Command::new("cargo")
        .arg("clean")
        .current_dir(dir.clone())
        .output()
        .expect("clean error!");

    info
}
