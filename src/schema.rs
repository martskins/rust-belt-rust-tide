table! {
    article (id) {
        id -> Int8,
        slug -> Varchar,
        title -> Varchar,
        description -> Text,
        tag_list -> Array<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
        favorites_count -> Int4,
        author_id -> Int8,
    }
}
