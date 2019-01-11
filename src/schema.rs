table! {
    categories (id) {
        id -> Integer,
        name -> Text,
    }
}

table! {
    comments (id) {
        id -> Integer,
        thread_id -> Integer,
        content -> Text,
        created_at -> Timestamp,
    }
}

table! {
    forums (id) {
        id -> Integer,
        name -> Text,
        category_id -> Integer,
        max_threads_number -> Integer,
        cur_threads_number -> Integer,
    }
}

table! {
    pictures (id) {
        id -> Integer,
        comment_id -> Integer,
        path_to_picture -> Text,
    }
}

table! {
    threads (id) {
        id -> Integer,
        forum_id -> Integer,
        title -> Text,
        position -> Integer,
        created_at -> Timestamp,
        bump -> Integer,
    }
}

joinable!(comments -> threads (thread_id));
joinable!(forums -> categories (category_id));
joinable!(pictures -> comments (comment_id));
joinable!(threads -> forums (forum_id));

allow_tables_to_appear_in_same_query!(categories, comments, forums, pictures, threads);
