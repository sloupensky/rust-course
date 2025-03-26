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

/// Gets user from database by its user_name
/// 
/// # Examples
/// ```
///  use data_utils::{get_user_by_name};
///  use data_utils::models::{User};
/// 
///  let Ok(user) = get_user_by_name("test".to_string()) else {
///     panic!("error while getting user");
///  };
///  user.name;
/// ```
pub fn get_user_by_name(user_name: String) -> Result<User, diesel::result::Error> {
    let mut connection = establish_connection();
    user.filter(name.eq(user_name))
        .select(User::as_select())
        .first(&mut connection)
}

/// Insert message into database
///
/// # Examples
/// ```
///  use data_utils::{insert_message};
///  use data_utils::models::{Message};
///
///  let Ok(message) = insert_message("text".to_string(), "file".to_string(), "image".to_string()) else {
///     panic!("error while saving message");
///  };
///  message.id;
/// ```
pub fn insert_message(text: String, file: String, image: String) -> Result<Message, diesel::result::Error> {
    let new_post = NewMessage { text, file, image };
    let mut connection = establish_connection();

    diesel::insert_into(message::table)
        .values(&new_post)
        .returning(Message::as_returning())
        .get_result(&mut connection)
}