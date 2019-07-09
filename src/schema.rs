table! {
    article (id) {
        id -> Int8,
        author_id -> Int8,
        description -> Text,
        favorites_count -> Int4,
        slug -> Varchar,
        tag_list -> Array<Text>,
        title -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
