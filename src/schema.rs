table! {
    entries (id) {
        id -> Integer,
        title -> Varchar,
        body -> Text,
        updated_at -> Datetime,
        created_at -> Datetime,
        published -> Bool,
        user_id -> Integer,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        uid -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    entries,
    users,
);
