use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use diesel::r2d2::{PooledConnection};
use crate::models::{Message, NewMessage, User};
use self::schema::user::dsl::*;
use self::schema::message::dsl::{message as dsl_message, user_id as dsl_user_id};
use crate::schema::{message, user};
use diesel::r2d2::{ ConnectionManager, Pool};

pub mod models;
pub mod schema;

pub type SqliteConnectionPool = Pool<ConnectionManager<SqliteConnection>>;
pub type SqlitePooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

fn get_connection_pool() -> SqliteConnectionPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    
    Pool::builder()
        .max_size(10)
        .build(manager)
        .expect("Failed to create SQLite connection pool")
}

pub fn get_connection() -> SqlitePooledConnection {
    get_connection_pool()
        .get()
        .expect("Failed to get connection from pool")
}

/// Gets user from database by its user_name
pub fn get_user_by_name(user_name: String, connection: &mut SqliteConnection) -> Result<User, diesel::result::Error> {
    user.filter(name.eq(user_name))
        .select(User::as_select())
        .first(connection)
}

pub fn get_all_users(connection: &mut SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
    user.load::<User>(connection)
}

pub fn get_filtered_users(user_id: i32, connection: &mut SqliteConnection) -> Result<Vec<User>, diesel::result::Error> {
    user.filter(id.eq(user_id)).load::<User>(connection)
}

pub fn delete_user_with_messages(target_user_id: i32, connection: &mut SqliteConnection) -> Result<(), diesel::result::Error> {
    diesel::delete(dsl_message.filter(dsl_user_id.eq(target_user_id)))
        .execute(connection)?;

    diesel::delete(user.filter(id.eq(target_user_id)))
        .execute(connection)?;
    
    Ok(())
}


pub fn get_all_messages(connection: &mut SqliteConnection) -> Result<Vec<(Message, User)>, diesel::result::Error> {
    message::table
        .inner_join(user::table)
        .select((Message::as_select(), User::as_select()))
        .load::<(Message, User)>(connection)
}

pub fn get_filtered_messages(user_id: i32, connection: &mut SqliteConnection) -> Result<Vec<(Message, User)>, diesel::result::Error> {
    message::table
        .inner_join(user::table)
        .filter(id.eq(user_id))
        .select((Message::as_select(), User::as_select()))
        .load::<(Message, User)>(connection)
}

/// Insert message into database
pub fn insert_message(text: String, file: String, image: String, user_id: i32, connection: &mut SqliteConnection) -> Result<Message, diesel::result::Error> {
    let new_post = NewMessage { text, file, image, user_id };

    diesel::insert_into(message::table)
        .values(&new_post)
        .returning(Message::as_returning())
        .get_result(connection)
}