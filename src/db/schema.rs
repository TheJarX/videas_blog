// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        content -> Text,
        slug -> Varchar,
        tags -> Array<Nullable<Text>>,
        description -> Text,
        created_at -> Timestamp,
    }
}
