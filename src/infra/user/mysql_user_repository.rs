use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use models::user::{User, UserRepository};
use schema::users;
use schema::users::dsl::*;

pub struct MySqlUserRepository<'a> {
    conn: &'a MysqlConnection,
}

impl<'a> MySqlUserRepository<'a> {
    pub fn new(conn: &'a MysqlConnection) -> MySqlUserRepository {
        MySqlUserRepository { conn: conn }
    }
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
    fn find_by_uid(&self, input_uid: &str) -> Result<User, DieselError> {
        users
            .select((
                users::id,
                users::name,
                users::uid,
                users::updated_at,
                users::created_at,
            )).filter(users::uid.eq(input_uid))
            .first::<User>(self.conn)
    }
}
