table! {
    chainkeys (chainkey, hash) {
        chainkey -> Binary,
        hash -> Binary,
        used -> Integer,
    }
}

table! {
    config (id) {
        id -> Text,
        colorscheme -> Integer,
        kp -> Binary,
        chk_id -> Nullable<Integer>,
    }
}

table! {
    contacts (user_id) {
        user_id -> Text,
        name -> Nullable<Text>,
        profile_picture -> Nullable<Text>,
        pairwise_conversation -> Binary,
        color -> Integer,
        status -> Integer,
        contact_type -> Integer,
        added -> Nullable<Integer>,
    }
}

table! {
    conversation_members (member_id, conversation_id) {
        member_id -> Text,
        conversation_id -> Binary,
    }
}

table! {
    conversations (conversation_id) {
        conversation_id -> Nullable<Binary>,
        title -> Nullable<Text>,
        picture -> Nullable<Text>,
        color -> Integer,
        muted -> Nullable<Integer>,
        pairwise -> Nullable<Integer>,
    }
}

table! {
    message_status (msg_id, conversation_id) {
        msg_id -> Binary,
        conversation_id -> Binary,
        status -> Nullable<Integer>,
    }
}

table! {
    messages (msg_id) {
        msg_id -> Binary,
        author -> Text,
        conversation_id -> Binary,
        body -> Text,
        attachment -> Nullable<Text>,
        op_msg_id -> Nullable<Integer>,
        timestamp -> Integer,
        expiration_date -> Nullable<Text>,
        send_status -> Nullable<Integer>,
    }
}

joinable!(contacts -> conversations (pairwise_conversation));
joinable!(conversation_members -> contacts (member_id));
joinable!(conversation_members -> conversations (conversation_id));
joinable!(message_status -> conversations (conversation_id));
joinable!(message_status -> messages (msg_id));
joinable!(messages -> contacts (author));
joinable!(messages -> conversations (conversation_id));

allow_tables_to_appear_in_same_query!(
    chainkeys,
    config,
    contacts,
    conversation_members,
    conversations,
    message_status,
    messages,
);
