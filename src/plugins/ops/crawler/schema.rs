table! {
    ops_crawler_logs (id) {
        id -> Int8,
        name -> Varchar,
        url -> Varchar,
        body -> Text,
        created_at -> Timestamp,
    }
}
