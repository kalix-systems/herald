use cbindgen::Config;

fn main() {
    let crate_dir = env!("CARGO_MANIFEST_DIR");
    let package_name = env!("CARGO_PKG_NAME");

    let config = Config {
        namespace: Some("ffi".into()),
        ..Default::default()
    };

    let output_file = format!("{}.h", package_name);

    cbindgen::generate_with_config(&crate_dir, config)
        .expect("Failed to generate bindings")
        .write_to_file(&output_file);
}
