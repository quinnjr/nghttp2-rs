# nghttp2-rs

nghttp2-rs provides higher level rust-friendly bindings to [nghttp2-sys](https://github.com/quinnjr/nghttp2-rs/tree/master/nghttp2).

nghttp2-rs is currently in its very early phase of development. Much of what the crate is intended to do is currently on hold until [RFC 2394](https://github.com/rust-lang/rust/issues/50547) is in a stable state or added to ```std``` . Otherwise, if all synchronous work is completed, nghttp2-rs will implement the async work with ```futures``` and/or ```tokio```.

## License

This crate is licensed under the [ISC License](LICENSE.md).

## Release Notes

- v0.1.1 - Adding nghttp2-sys low-level bindings to nghttp2-rs workspace. Correcting version constraints for dependencies.
- v0.1.0 - Initial release.