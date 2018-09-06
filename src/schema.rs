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
    tokens (id) {
        id -> Integer,
        user_id -> Integer,
        token -> Text,
        updated_at -> Datetime,
        created_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Varchar,
        uid -> Varchar,
        created_at -> Datetime,
        updated_at -> Datetime,
    }
}

allow_tables_to_appear_in_same_query!(entries, tokens, users,);
