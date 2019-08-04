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

table! {
    user (id) {
        id -> Int8,
        name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        password -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(article -> user (author_id));

allow_tables_to_appear_in_same_query!(
    article,
    user,
);
