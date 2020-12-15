table! {
    schema_migrations (id) {
        id -> Integer,
        version -> Bpchar,
        name -> Varchar,
        up -> Text,
        down -> Text,
        run_at -> Nullable<Timestamp>,
    }
}
