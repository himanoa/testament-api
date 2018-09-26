use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use infra::mysql::DbConn;
use schema::users;
use schema::tokens;
use schema::tokens::dsl::*;
use models::user::User;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct Token {
    pub id: i32,
    pub user_id: i32,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize, Insertable, Associations)]
#[table_name = "tokens"]
#[belongs_to(User)]
pub struct NewToken {
    pub user_id: i32,
    pub token: String,
}

pub trait TokenRepository {
    fn resolve(&self, id: i32) -> Result<Token, DieselError>;
    fn create(&self, user: User) -> Result<i32, DieselError>;
    fn delete(&self, id: i32) -> Result<Token, DieselError>;
    fn delete_all_by_user(&self, user: User) -> Result<i32, DieselError>;
    fn list_by_user(&self, user: User) -> Result<Token, DieselError>;
}

pub struct MySqlTokenRepository<'a> {
    conn: &'a MysqlConnection,
}

impl<'a> MySqlTokenRepository<'a> {
    pub fn new(conn: &'a MySqlTokenRepository) -> MySqlTokenRepository {
        MySqlTokenRepository { conn: conn }
    }
}

impl<'a> Token for MySqlTokenRepository<'a> {

}
