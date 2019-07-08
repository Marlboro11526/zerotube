table! {
    confirmation_emails (id) {
        id -> Text,
        expiry_date_time -> Timestamp,
        user_id -> Text,
    }
}

table! {
    rooms (id) {
        id -> Text,
        name -> Text,
        public -> Bool,
    }
}

table! {
    users (id) {
        id -> Text,
        active -> Bool,
        email -> Text,
        password -> Text,
        username -> Text,
    }
}

joinable!(confirmation_emails -> users (user_id));

allow_tables_to_appear_in_same_query!(
    confirmation_emails,
    rooms,
    users,
);
