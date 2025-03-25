use diesel::prelude::*;

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::user)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct User {
    pub id: i32,
    pub name: String,
}
#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::message)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct Message {
    pub id: i32,
    pub text: String,
    pub file: String,
    pub image: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::message)]
pub struct NewMessage {
    pub text: String,
    pub file: String,
    pub image: String,
}