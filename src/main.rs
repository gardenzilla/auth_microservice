#![feature(test)]
extern crate test;

use packman::*;
use std::{env, path::PathBuf, sync::Arc};
use tokio::sync::{oneshot, Mutex};
use tokio::time::{sleep, Duration};

mod auth;
mod hasher;
mod prelude;

struct AuthService {
  tokens: Arc<Mutex<VecPack<auth::AuthObject>>>,
  hasher: hasher::Hasher,
}

impl AuthService {
  fn init(db: VecPack<auth::AuthObject>, hasher: hasher::Hasher) -> Self {
    Self {
      tokens: Arc::new(Mutex::new(db)),
      hasher,
    }
  }
  async fn cleaner(&self) {
    let tokens = self.tokens.clone();
    tokio::spawn(async move {
      let tokens = tokens;
      loop {
        // Collect token IDs to remove
        let ids_to_remove = tokens
          .lock()
          .await
          .iter()
          .filter(|i| {
            i.unpack()
              .is_outdated(chrono::Duration::seconds(90 * 24 * 60 * 60)) // 90 days
          })
          .map(|i| *i.unpack().get_id())
          .collect::<Vec<u32>>();

        // Remove tokens
        for id in ids_to_remove {
          let _ = tokens.lock().await.remove_pack(&id);
        }

        // Wait 1 secs
        sleep(Duration::from_secs(1)).await;
      }
    });
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

  let mut db: VecPack<auth::AuthObject> = VecPack::load_or_init(PathBuf::from("data/auth_tokens"))
    .expect("Error while loading auth_tokens db");

  (0..1000)
    .into_iter()
    .for_each(|i| db.insert(auth::AuthObject::init(i)).unwrap());

  let auth_service = AuthService::init(db, hasher);

  auth_service.cleaner().await;

  println!("Hello..");

  // Wait 10 minutes
  sleep(Duration::from_secs(60 * 10)).await;

  Ok(())
}
