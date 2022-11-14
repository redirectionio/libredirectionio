# About

rust library that process redirect and filtering from rules coming of https://redirection.io/

This library is mainly used in proxies and agent of redirectionio.

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

## Tests

Some tests are generated. Templates are located in `tests/templates` folder. If
you update them, you need to run `cargo build` to (re)generate the tests.
