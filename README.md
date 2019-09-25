# About

rust library that process redirect and filtering from rules coming of https://redirection.io/

## Installation

You only to install this library if you wish to compile apache or nginx proxies or build the web assembly module.

### Library

You need rust and cargo to compile this library, to install on your system run the following commands:

```
autoreconf -i
./configure
make
```

You can run `make install` (with root permissions) to install this library into your system, this is required if you need to 
compile some of our modules against this library.

### Web Assembly

To build the web assembly module you will need `wasm-pack` and then run:

```
wasm-pack build
```

## License

This code is licensed under the MIT License - see the  [LICENSE](./LICENSE.md)  file for details.
