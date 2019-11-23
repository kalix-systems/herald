#[macro_use]
extern crate rust_qt_binding_generator;

pub mod config;
use rust_qt_binding_generator::generate_bindings;

fn main() {
    let conf = config::get();

    generate_bindings(&conf).expect("failed to generate bindings");
}
