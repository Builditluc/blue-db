table! {
    categories (id) {
        id -> Text,
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    entries (id) {
        id -> Text,
        category_id -> Text,
        title -> Varchar,
        body -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    categories,
    entries,
);
