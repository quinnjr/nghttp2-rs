// Copyright (c) 2018 Joseph R. Quinn <quinn.josephr@protonmail.com>
// SPDX-License-Identifier: ISC

extern crate bindgen;

use std::{env, path::PathBuf, process::Command, string::String};

fn main() {
  // Print link-lib command to stdout.
  println!("cargo:rustc-link-lib=libnghttp2");

  let rustfmt_path = if cfg!(feature = "rustup") {
    let output = Command::new("rustup")
      .arg("which")
      .arg("rustfmt")
      .output()
      .expect("Failed to execute `rustup which rustfmt`");

    PathBuf::from(String::from_utf8(output.stdout).unwrap())
  } else if cfg!(not(feature = "rustup")) && cfg!(unix) {
    let output = Command::new("which")
      .arg("rustfmt")
      .output()
      .expect("Failed to find rustfmt in $PATH");

    PathBuf::from(String::from_utf8(output.stdout).unwrap())
  } else {
    let output = Command::new("where.exe")
      .arg("rustfmt")
      .output()
      .expect("Faild to find rustfmt in %PATH%");

    PathBuf::from(String::from_utf8(output.stdout).unwrap())
  };

  // Build the bindings Builder struct.
  let builder = if cfg!(feature = "rustfmt") {
    bindgen::Builder::default()
      .whitelist_type("nghttp2_.*")
      .whitelist_function("nghttp2_.*")
      .whitelist_var("nghttp2_.*")
      .generate_comments(false)
      .with_rustfmt(rustfmt_path)
      .rustfmt_configuration_file(Some(PathBuf::from("../rustfmt.toml")))
      .rustfmt_bindings(true)
      .header("src/wrapper.h")
  } else {
    bindgen::Builder::default()
      .whitelist_type("nghttp2_.*")
      .whitelist_function("nghttp2_.*")
      .whitelist_var("nghttp2_.*")
      .generate_comments(false)
      .rustfmt_bindings(false)
      .header("src/wrapper.h")
  };

  // Generate the bindings.
  let bindings = builder.generate().expect("Failed to generate bindings.");

  // Determine the out_path for the bindings file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

  // Writing bindings to OUT directory for the compiled library.
  bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Could not write the bindings file.");
}
