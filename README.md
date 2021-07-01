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

### Web Assembly

To build the web assembly module you will need `wasm-pack` and then run:

```
wasm-pack build
```

### Pushing to cloudflare

You can directly push this code to a cloudflare worker, but you still need a redirection io account to do so:

 1. You need to have wrangler installed: `cargo install wrangler`
 2. Login or configure your api token for cloudflare `wrangler login` or `wrangler config`
 3. Copy file `wrangler.toml.dist` to `wrangler.toml` and replace needed value
 4. Push your redirection io token as a secret value `wrangler secret put REDIRECTIONIO_TOKEN` and enter you redirection io project key when asked (available in your the instance panel of your project)
 5. Publish your worker: `wrangler publish`

## License

This code is licensed under the MIT License - see the  [LICENSE](./LICENSE.md)  file for details.
