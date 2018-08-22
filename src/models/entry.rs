use schema::{entries};
use models::user::{User};
use chrono::{NaiveDateTime};

#[derive(Debug, Serialize, Deserialize, Queryable, Associations)]
#[belongs_to(User)]
#[table_name="entries"]
pub struct Entry {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
    pub published: bool,
}

#[derive(Debug, Serialize, Deserialize, Insertable)]
#[table_name="entries"]
pub struct NewEntry {
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub published: bool
}
