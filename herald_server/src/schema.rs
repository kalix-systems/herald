table! {
    keys (key) {
        key -> Bytea,
        signed_by -> Bytea,
        ts -> Timestamptz,
        signature -> Bytea,
        dep_ts -> Nullable<Timestamptz>,
        dep_signed_by -> Nullable<Bytea>,
        dep_signature -> Nullable<Bytea>,
    }
}

table! {
    pending (key, push_id) {
        key -> Bytea,
        push_id -> Int8,
    }
}

table! {
    prekeys (sealed_key) {
        sealed_key -> Bytea,
        signing_key -> Bytea,
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
        key -> Bytea,
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
