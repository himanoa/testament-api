#![allow(proc_macro_derive_resolution_fallback)]
use chrono::NaiveDateTime;
use diesel::result::Error as DieselError;
use models::user::User;
use schema::tokens;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct NewToken {
    pub user_id: i32,
    pub token: String,
}

pub trait TokenRepository {
    fn resolve(&self, token: String) -> Result<Token, DieselError>;
    fn create(&self, new_token: &NewToken) -> Result<usize, DieselError>;
    fn delete(&self, target_id: i32) -> Result<usize, DieselError>;
    fn delete_all_by_user(&self, user: &User) -> Result<usize, DieselError>;
    fn list_by_user(&self, user: &User) -> Result<Vec<Token>, DieselError>;
}
