# nghttp2-sys
[![Build Status](https://travis-ci.com/quinnjr/nghttp2-sys.svg?branch=master)](https://travis-ci.com/quinnjr/nghttp2-sys)

Rust raw FFI bindings to the http2 implementation provided by [nghttp2](https://nghttp2.org). Bindings are generated with [bindgen](https://crates.io/crates/bindgen).

## Requirements
`nghttp2` library >= [1.0.0](https://github.com/nghttp2/nghttp2/tree/master)

## License

This crate is licensed under the [ISC License](LICENSE.md).

## Release Notes

- v0.1.1 - Moving crate into a workspace alongside [nghttp2-rs](https://github.com/quinnjr/nghttp2-rs). Loosening bindgen version constraint. Adding missing editorconfig.
- v0.1.0 - Initial release.