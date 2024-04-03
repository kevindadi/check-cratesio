use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::crate_infos)]
pub struct CrateInfo {
    pub id: i32,
    pub name: String,
    pub path: String,
    pub result: String,
}

impl CrateInfo {
    pub fn new(id: i32, name: String, path: String) -> Self {
        Self {
            id,
            name,
            path,
            result: String::new(),
        }
    }
}

#[derive(Insertable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = crate::schema::crate_results)]
pub struct CrateResult {
    pub compile_pass: bool,
    pub no_deadlock: bool,
    pub reason: Option<String>,
}

impl CrateResult {
    pub fn new(compile_pass: bool) -> Self {
        Self {
            compile_pass,
            no_deadlock: true,
            reason: None,
        }
    }
}

// impl Into<CrateInfo> for (i32, String, Option<String>) {
//     fn into(self) -> CrateInfo {
//         CrateInfo {
//             id: self.0,
//             name: self.1,
//             result: self.2,
//         }
//     }
// }
