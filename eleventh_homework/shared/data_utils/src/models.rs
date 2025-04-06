use diesel::prelude::*;
use serde::Serialize;

#[derive(Queryable, Identifiable, Selectable, Debug, Serialize, PartialEq)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}
#[derive(Queryable, Selectable, Serialize, PartialEq, Debug, Identifiable, Associations)]
#[diesel(table_name = crate::schema::message)]
#[diesel(belongs_to(User))]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub file: String,
    pub image: String,
    pub user_id: i32,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(belongs_to(User))]
pub struct NewMessage {
    pub text: String,
    pub file: String,
    pub image: String,
    pub user_id: i32,
}
