// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

use http::Method;

pub struct Request<'a> {
  url: &'a str,
  method: Method,
}

impl<'a> Request<'a> {
  pub fn new(url: &'a str, method: Method) -> Request<'a> {
    Request {
      url,
      method
    }
  }
}
