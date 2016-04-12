extern crate rustc_serialize;

use std::process::*;
use std::env::args;
use rustc_serialize::json;

#[derive(RustcEncodable)]
struct Case {
    input: String,
    expect: String,
}

#[derive(RustcEncodable)]
struct Problem {
    cases: Vec<Case>,
}

fn main() {
}

