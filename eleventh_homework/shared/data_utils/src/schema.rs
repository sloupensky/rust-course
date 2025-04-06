// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Integer,
        text -> Text,
        image -> Text,
        file -> Text,
        user_id -> Integer,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::joinable!(message -> user (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    message,
    user,
);
