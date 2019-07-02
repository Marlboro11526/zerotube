table! {
    confirmation_emails (id) {
        id -> Text,
        user_id -> Text,
        expiry_date_time -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Text,
        email -> Text,
        username -> Text,
        password -> Text,
        active -> Bool,
    }
}

joinable!(confirmation_emails -> users (user_id));

allow_tables_to_appear_in_same_query!(
    confirmation_emails,
    users,
);
