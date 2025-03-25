// @generated automatically by Diesel CLI.

diesel::table! {
    message (id) {
        id -> Integer,
        text -> Text,
        image -> Text,
        file -> Text,
    }
}

diesel::table! {
    user (id) {
        id -> Integer,
        name -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    message,
    user,
);
