table! {
    confirmation_emails (id) {
        id -> Text,
        expiry_date_time -> Timestamp,
        user_id -> Text,
    }
}

table! {
    room_media (id) {
        id -> Text,
        room_id -> Text,
        source -> Text,
        name -> Text,
        room_media_index -> Integer,
        seconds -> Integer,
        url -> Text,
    }
}

table! {
    rooms (id) {
        id -> Text,
        description -> Text,
        name -> Text,
        public -> Bool,
        url -> Text,
    }
}

table! {
    sources (name) {
        name -> Text,
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
joinable!(room_media -> rooms (room_id));
joinable!(room_media -> sources (source));

allow_tables_to_appear_in_same_query!(
    confirmation_emails,
    room_media,
    rooms,
    sources,
    users,
);
