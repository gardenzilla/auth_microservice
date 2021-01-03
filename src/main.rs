#![feature(test)]
extern crate test;

use packman::*;
use std::env;

mod auth;
mod hasher;

struct AuthService {
  tokens: VecPack<auth::AuthObject>,
  hasher: Hasher,
}

fn main() {
  let hasher = hasher::Hasher::init(
    env::var("HASH_SECRET")
      .unwrap_or("SECRET".into())
      .as_bytes()
      .to_owned(),
  );
}
