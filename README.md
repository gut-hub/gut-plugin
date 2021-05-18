# gut-plugin
Helper library that provides a macro to create plugins for [gut](https://github.com/gut-hub/gut).

The gut plugin system loads plugins that are written in `wasm` that are located in the gut directory `$HOME/.gut`.

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
use std::slice;
use std::str;

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
// fn_name(ptr: i32, len: i32)
//
// this is becuase gut will invoke the function and pass a string if one is provided.
// example:
// $ gut my_plugin "world"
#[no_mangle]
fn my_plugin(ptr: i32, len: i32) {
  let slice = unsafe { slice::from_raw_parts(ptr as _, len as _) };
  let string_from_host = str::from_utf8(&slice).unwrap();

  println!("Hello {}!", string_from_host)
}
```

4. Build
```sh
# you may need to add wasm32-wasi target to build wasm
$ rustup target add wasm32-wasi

# build
$ cargo build --target wasm32-wasi --release

# move to gut directory
$ cp target/wasm32-wasi/release/gut_myplugin.wasm $HOME/.gut/
```
