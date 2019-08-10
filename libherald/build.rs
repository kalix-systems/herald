use std::process::Command;

fn main() {
    Command::new("cbindgen")
        .args(&[".", "-o", "herald.h"])
        .spawn()
        .expect("Failed to generate bindings");
}
