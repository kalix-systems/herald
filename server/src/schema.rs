table! {
    keys (key) {
        key -> Bit,
        signed_by -> Bit,
        creation_ts -> Timestamp,
        signature -> Bit,
        deprecation_ts -> Nullable<Timestamp>,
        dep_signed_by -> Nullable<Bit>,
        dep_signature -> Nullable<Bit>,
    }
}

table! {
    pending (key, push_id) {
        key -> Bit,
        push_id -> Int8,
    }
}

table! {
    prekeys (signing_key) {
        signing_key -> Bit,
        sealed_key -> Bytea,
    }
}

table! {
    pushes (push_id) {
        push_id -> Int8,
        push_data -> Bytea,
    }
}

table! {
    userkeys (user_id, key) {
        user_id -> Bpchar,
        key -> Bit,
    }
}

joinable!(pending -> keys (key));
joinable!(pending -> pushes (push_id));
joinable!(prekeys -> keys (signing_key));
joinable!(userkeys -> keys (key));

allow_tables_to_appear_in_same_query!(
    keys,
    pending,
    prekeys,
    pushes,
    userkeys,
);
