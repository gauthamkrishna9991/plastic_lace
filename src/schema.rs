table! {
    admins (id) {
        id -> Int4,
        username -> Varchar,
        display_name -> Text,
        contact -> Text,
        passwd -> Text,
    }
}

table! {
    attendance_logs (id) {
        id -> Int8,
        admin -> Int4,
        checkin -> Nullable<Timestamp>,
        checkout -> Nullable<Timestamp>,
    }
}

allow_tables_to_appear_in_same_query!(
    admins,
    attendance_logs,
);
