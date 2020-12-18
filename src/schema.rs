table! {
    categories (id) {
        id -> Int4,
        title -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    entries (id) {
        id -> Int4,
        entry_id -> Text,
        category_id -> Int4,
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
