table! {
    ops_cron_tasks (id) {
        id -> Int8,
        schedule -> Varchar,
        user -> Nullable<Varchar>,
        command -> Varchar,
        version -> Int8,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
