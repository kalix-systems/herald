const NUM_COLORS: u64 = 9;

pub fn id_to_color(id: &str) -> u32 {
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    let mut state = DefaultHasher::default();
    id.hash(&mut state);
    (state.finish() % NUM_COLORS) as u32
}
