This project uses the crate `qt_bindgen` to generate bindings so Rust
code can be easily used from C++.

*Dependencies*:

* `cargo-make`
* `clang` (for `clang-format`)


Run:

```bash
cargo make bindgen
```

to generate the bindings.
