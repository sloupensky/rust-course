use prometheus::{Encoder, Registry, TextEncoder};
use crate::DbConn;
use data_utils;
use data_utils::models::{Message, User};
use rocket::form::Form;
use rocket_dyn_templates::{context, Template};
use serde::Serialize;
use rocket::State;


#[derive(FromForm)]
pub struct FilterForm {
    user_id: i32,
}

#[derive(Serialize)]
struct Users {
    filtered_users: Vec<User>,
    all_users: Vec<User>,
    selected_user: i32,
}

#[derive(Serialize)]
struct MessageWithUser {
    message: Message,
    user: User,
}
#[derive(Serialize)]
struct Messages {
    messages: Vec<MessageWithUser>,
    users: Vec<User>,
    selected_user: i32,
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context!())
}

#[get("/metrics")]
fn metrics(registry: &State<Registry>) -> String {
    let encoder = TextEncoder::new();
    let mut buffer = vec![];

    encoder.encode(&registry.gather(), &mut buffer).expect("failed to encode metrics");
    
    String::from_utf8(buffer).expect("Failed to transform buffer into utf8")
}

#[get("/users")]
async fn get_users(connection: DbConn) -> Template {
    let filtered_users = connection
        .run(|mut c| data_utils::get_all_users(&mut c))
        .await
        .unwrap();
    let all_users = connection
        .run(|mut c| data_utils::get_all_users(&mut c))
        .await
        .unwrap();

    Template::render(
        "users",
        Users {
            filtered_users,
            all_users,
            selected_user: 0,
        },
    )
}

#[post("/users/filter", data = "<filter_form>")]
async fn get_filtered_users(filter_form: Form<FilterForm>, connection: DbConn) -> Template {
    let selected_user = filter_form.user_id.clone();
    let filtered_users = connection
        .run(move |mut c| data_utils::get_filtered_users(filter_form.user_id.clone(), &mut c))
        .await
        .unwrap();

    let all_users = connection
        .run(|mut c| data_utils::get_all_users(&mut c))
        .await
        .unwrap();

    Template::render(
        "users",
        Users {
            filtered_users,
            all_users,
            selected_user: selected_user,
        },
    )
}

#[get("/messages")]
async fn get_messages(connection: DbConn) -> Template {
    let result = connection
        .run(|mut c| data_utils::get_all_messages(&mut c))
        .await
        .unwrap();
    let messages_with_user: Vec<MessageWithUser> = result
        .into_iter()
        .map(|(message, user)| MessageWithUser { message, user })
        .collect();
    
    let users = connection
        .run(|mut c| data_utils::get_all_users(&mut c))
        .await
        .unwrap();
    
    Template::render(
        "messages",
        Messages {
            messages: messages_with_user,
            users,
            selected_user: 0
        },
    )
}
#[post("/messages/filter", data = "<filter_form>")]
async fn get_filtered_messages(filter_form: Form<FilterForm>, connection: DbConn) -> Template {
    let selected_user = filter_form.user_id.clone();
    let result = connection
        .run(move |mut c| data_utils::get_filtered_messages(filter_form.user_id, &mut c))
        .await
        .unwrap();
    let messages_with_user: Vec<MessageWithUser> = result
        .into_iter()
        .map(|(message, user)| MessageWithUser { message, user })
        .collect();

    let users = connection
        .run(|mut c| data_utils::get_all_users(&mut c))
        .await
        .unwrap();

    Template::render(
        "messages",
        Messages {
            messages: messages_with_user,
            users,
            selected_user
        },
    )
}

#[get("/user/delete/<user_id>")]
async fn delete_user(user_id: i32, connection: DbConn) -> Template {
    let _ = connection
        .run(move |conn| data_utils::delete_user_with_messages(user_id, conn))
        .await;

    Template::render("user_deleted", context!())
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        index,
        get_users,
        get_messages,
        delete_user,
        get_filtered_users,
        get_filtered_messages,
        metrics
    ]
}
