# gut-plugin
Helper library that provides a macro to create plugins for [gut](https://github.com/gut-hub/gut).


# How to create a gut plugin

1. Create a new library using cargo
```sh
$ cargo new my_gut_plugin --lib
```

2. Add the following to the `Cargo.toml`:
```toml
[lib]
name = "gut_myplugin"
crate-type = ["cdylib"]

[dependencies]
gut-plugin = "0.1.0"
```

3. Use the provided macro to export the function
```rust
// gut_export!([names], [descriptions])
// names: [&str] - the names of the functions to export.
// descriptions: [&str] - the descriptions of the functions to export.
gut_plugin::gut_export!(
  ["my_plugin"],
  ["Prints the provided string"]
);

// all exported functions must have this signature:
//
// #[no_mangle]
// fn_name(*mut c_char)
//
// this is becuase gut will invoke the function and pass a string if one is provided.
// example:
// $ gut my_plugin "world"
#[no_mangle]
fn my_plugin(ptr: *mut c_char) {
  let c_string = unsafe { CString::from_raw(ptr) };
  let str_from_host = c_string.to_str().unwrap();

  println!("Hello {}!", string_from_host)
}
```

4. Build
```sh
# add wasm32-wasi target if you want to build to wasm
$ rustup target add wasm32-wasi

# build native or wasm
$ cargo build --release
$ cargo build --target wasm32-wasi --release

# move to gut directory
$ cp target/release/libgut_myplugin.dylib $HOME/.gut/
$ cp target/wasm32-wasi/release/gut_myplugin.wasm $HOME/.gut/
```
