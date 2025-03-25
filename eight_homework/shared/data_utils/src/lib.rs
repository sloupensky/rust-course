use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::models::{Message, NewMessage, User};
use self::schema::user::dsl::*;
use crate::schema::message;

pub mod models;
pub mod schema;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| {
            eprintln!("Error connecting to {}", database_url);
            std::process::exit(0);
        })
}

pub fn get_user_by_name(user_name: String) -> Result<User, diesel::result::Error> {
    let mut connection = establish_connection();
    user.filter(name.eq(user_name))
        .select(User::as_select())
        .first(&mut connection)
}

pub fn insert_message(text: String, file: String, image: String) -> Result<Message, diesel::result::Error> {
    let new_post = NewMessage { text, file, image };
    let mut connection = establish_connection();

    diesel::insert_into(message::table)
        .values(&new_post)
        .returning(Message::as_returning())
        .get_result(&mut connection)
}