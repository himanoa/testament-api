use chrono::NaiveDateTime;
use diesel;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use infra::mysql::DbConn;
use schema::users;
use schema::users::dsl::*;

#[derive(Debug, Serialize, Deserialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub name: String,
    pub uid: String,
    pub updated_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}

pub trait UserRepository {
    fn resolve(&self, id: i32) -> Result<User, DieselError>;
    // fn find_by_uid(&self, uid: &str) -> Option<User>;
}

pub struct MySqlUserRepository<'a> {
    conn: &'a MysqlConnection,
}

impl<'a> UserRepository for MySqlUserRepository<'a> {
    fn resolve(&self, user_id: i32) -> Result<User, DieselError> {
        users
            .select((
                users::id,
                users::name,
                users::uid,
                users::updated_at,
                users::created_at,
            )).find(user_id)
            .first::<User>(self.conn)
    }
    // fn find_by_uid(&self, input_uid: &str) -> Option<User> {
    //     users::table.
    // }
}
