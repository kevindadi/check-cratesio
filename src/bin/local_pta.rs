use core::time;
use std::{io::Read, process::Stdio, thread};

use clap::{App, Arg};
use tokio::process::Command;
use walkdir::WalkDir;

fn parse_args() -> String {
    let matches = App::new("Cargo PTA Runner")
        .version("1.0")
        .author("Kevin")
        .about("Runs `cargo pta` on all subdirectories")
        .arg(
            Arg::with_name("path")
                .help("The path to the folder containing projects")
                .required(true)
                .index(1),
        )
        .get_matches();

    matches.value_of("path").unwrap().to_string()
}

async fn run_cargo_pta_in_subfolders(path: String) {
    for entry in WalkDir::new(path)
        .min_depth(1)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_dir())
    {
        let test_path = entry.path();
        println!("Running `cargo pta` in: {:?}", test_path.display());

        let output = Command::new("cargo")
            .arg("pta")
            .current_dir(test_path)
            .output()
            .await
            .expect("failed to execute process");

        // 输出到控制台或保存到文件
        println!("Status: {}", output.status);
        if !output.stdout.is_empty() {
            println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
        }
        if !output.stderr.is_empty() {
            println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
    }
}

// #[tokio::main]
fn main() {
    // let path = parse_args();
    // run_cargo_pta_in_subfolders(path).await;
    test_cargo_pta_output();
}

pub fn test_cargo_pta_output() {
    let output = std::process::Command::new("cargo")
        .arg("pta")
        .current_dir("/home/kevin/burble/")
        // .output()
        .output()
        .expect("nononon");
    //thread::sleep(time::Duration::from_secs(15));
    // let output = std::process::Command::new("/bin/cat")
    //     .arg("file.txt")
    //     .output()
    //     .expect("failed to execute process");
    //println!("{:?}", output);
    // if !output.stdout.is_empty() {
    //     if String::from_utf8_lossy(&output.stdout).contains("Lock") {
    //         println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    //         println!("contain lock ");
    //     }
    //     println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    //     println!("stdout no lock ");
    // } else {
    //     println!("stdout is empty");
    // }
    if !output.stderr.is_empty() {
        if String::from_utf8_lossy(&output.stderr).contains("Lock") {
            println!("Stdout: {}", String::from_utf8_lossy(&output.stderr));
            println!("contain lock ");
        }
        println!("failed ");
    }
}
