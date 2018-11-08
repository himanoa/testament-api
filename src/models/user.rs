#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub uid: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
    pub uid: String,
}

pub trait UserRepository {
    fn resolve(&self, id: i32) -> Result<User, DieselError>;
    fn find_by_uid(&self, uid: &str) -> Result<User, DieselError>;
}
