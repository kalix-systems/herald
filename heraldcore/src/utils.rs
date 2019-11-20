use coretypes::ids::UID_LEN;

const NUM_COLORS: u64 = 9;

pub(crate) fn id_to_color<H: std::hash::Hash>(id: H) -> u32 {
    use std::{collections::hash_map::DefaultHasher, hash::Hasher};

    let mut state = DefaultHasher::default();
    id.hash(&mut state);
    (state.finish() % NUM_COLORS) as u32
}

pub(crate) fn rand_id() -> [u8; UID_LEN] {
    use sodiumoxide::randombytes::randombytes_into;
    if sodiumoxide::init().is_err() {
        eprintln!("failed to init libsodium - what have you done");
        std::process::abort()
    }

    let mut buf = [0u8; UID_LEN];
    randombytes_into(&mut buf);
    buf
}
