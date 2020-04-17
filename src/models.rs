use super::schema::snippets;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
#[derive(Queryable, Serialize, Insertable, Debug, Clone)]
#[table_name = "snippets"]
pub struct Snippet {
    pub id: i32,
    pub title: String,
    pub content: String,
    pub created: NaiveDateTime,
    pub expires: NaiveDateTime,
}
#[derive(Insertable, Deserialize, Serialize, Debug, Clone)]
#[table_name = "snippets"]
pub struct NewSnippet {
    pub title: String,
    pub content: String,
}
