#![feature(trivial_bounds)]

use diesel::prelude::*;
use std::time::SystemTime;

// 为TsVector实现FromSql，以便可以从数据库读取Tsvector类型

// #[derive(Queryable)] 将生成从SQL查询加载Post结构所需的所有代码
// #[derive(Selectable)] 将生成代码,以基于通过
// #[diesel(table_name = crate::schema::posts)] 定义的表的模型类型构造匹配的 select 子句
// #[diesel(check_for_backend(diesel::pg::Pg)) 添加额外的编译时检查,以验证结构中的所有字段类型与其相应的SQL端表达式兼容
// 这部分是可选的，但它极大地改善了生成的编译器错误消息
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::crates)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Crate {
    pub id: i32,
    pub name: String,
    pub updated_at: SystemTime,
    pub created_at: SystemTime,
    pub downloads: i32,
    pub description: Option<String>,
    pub homepage: Option<String>,
    pub documentation: Option<String>,
    pub readme: Option<String>,
    // pub textsearchable_index_col: Tsvector,
    pub repository: Option<String>,
    pub max_upload_size: Option<i32>,
    pub max_features: Option<i16>,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::crates_categories)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Category {
    pub crate_id: i32,
    pub category_id: i32,
}
