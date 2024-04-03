pub mod crate_info;
pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connection to {}", database_url))
}

// #[derive(Debug, Deserialize, Serialize)]
// pub struct Config {
//     #[serde(default)]
//     pub pg: deadpool_postgres::Config,
// }

// impl Config {
//     pub fn from_env() -> Result<Self, config::ConfigError> {
//         config::Config::builder()
//             .add_source(config::Environment::default().separator("__"))
//             .build()
//             .unwrap()
//             .try_deserialize()
//     }
// }

// pub fn create_post(conn: &mut PgConnection, title: &str, body: &str) -> Post {
//     use crate::schema::posts;

//     let new_post = NewPost { title, body };

//     diesel::insert_into(posts::table)
//         .values(&new_post)
//         .returning(Post::as_returning())
//         .get_result(conn)
//         .expect("Error saving new post")
// }
