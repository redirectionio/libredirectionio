# About

rust library that process redirect and filtering from rules coming of https://redirection.io/

## Installation

You only to install this library if you wish to compile apache or nginx proxies or build the web assembly module.

You will need rust to compile this library, then simply run the following command:

```
cargo build --release
```

This will produce a `redirectionio.h` header file (inside the `target` directory) 
and library files `.a` and `.so` inside the `target/release` directory

## Web Assembly

To build the web assembly module you will need `wasm-pack` then simply run:

```
wasm-pack build
```

## License

This code is licensed under the MIT License - see the  [LICENSE](./LICENSE.md)  file for details.
