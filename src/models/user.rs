use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub uid: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
