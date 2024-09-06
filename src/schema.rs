// @generated automatically by Diesel CLI.

diesel::table! {
    todos (user_id, todo_id) {
        user_id -> Int4,
        todo_id -> Int4,
        creation_dt -> Date,
        active_status -> Bool,
        description -> Text,
        set_dt -> Date,
        color -> Text,
    }
}

diesel::table! {
    users (user_id) {
        #[max_length = 40]
        email -> Varchar,
        user_id -> Int2,
        #[max_length = 30]
        user_name -> Varchar,
        salt_hash_bytes -> Bytea,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    todos,
    users,
);
