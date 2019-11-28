#[macro_use]
extern crate riqtshaw;

pub mod config;
use riqtshaw::generate_bindings;

fn main() {
    let conf = config::get();

    generate_bindings(&conf).expect("failed to generate bindings");
}
