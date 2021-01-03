#![feature(test)]
extern crate test;

use packman::*;
use std::{env, path::PathBuf};
use tokio::sync::{oneshot, Mutex};

mod auth;
mod hasher;
mod prelude;

struct AuthService {
  tokens: Mutex<VecPack<auth::AuthObject>>,
  hasher: hasher::Hasher,
}

impl AuthService {
  fn init(db: VecPack<auth::AuthObject>, hasher: hasher::Hasher) -> Self {
    Self {
      tokens: Mutex::new(db),
      hasher,
    }
  }
}

#[tokio::main]
async fn main() -> prelude::ServiceResult<()> {
  let hasher = hasher::Hasher::init(
    env::var("HASH_SECRET")
      .expect("NO HASH_SECRET ENV FOUND")
      .as_bytes()
      .to_owned(),
  );

  let db: VecPack<auth::AuthObject> = VecPack::load_or_init(PathBuf::from("data/auth_tokens"))
    .expect("Error while loading auth_tokens db");

  let auth_service = AuthService::init(db, hasher);

  Ok(())
}
