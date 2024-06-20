table! {
    tasks (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Text,
        status -> Varchar,
        due_date -> Nullable<Timestamp>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Varchar,
        role -> Varchar,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    tasks,
    users,
);
