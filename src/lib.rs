// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

#![allow(unused_imports, dead_code)]

// 3rd party dependencies.
extern crate http;
extern crate libc;
extern crate nghttp2_sys as nghttp2;
#[macro_use]
extern crate slog;
extern crate futures;

#[cfg(test)]
extern crate futures_cpupool;

// Export internal modules.
pub mod client;
pub mod proxy;
pub mod server;

#[cfg(test)]
mod tests {
  use super::*;

  use futures::Future;
  use futures_cpupool::CpuPool;
  use http::Method;

  #[test]
  fn http_client_future() {
    let _pool = CpuPool::new(4);

    // let response = pool.spawn(client::Request::new("https://google.com", Method::GET));
  }
}
