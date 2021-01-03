use std::time::Instant;

use sha2::{Digest, Sha512};
use uuid::Uuid;

/// Hasher struct
/// holding the secret required
/// to create and verify token
pub struct Hasher {
  secret: Vec<u8>,
}

impl Hasher {
  /// Init hasher with a giver secret
  pub fn init(secret: Vec<u8>) -> Self {
    Self { secret }
  }
  // Create hash from a base byte array
  fn create_hash(&self, base: &[u8]) -> Vec<u8> {
    // create a Sha256 object
    let mut hasher = Sha512::new();
    // write base
    hasher.update(base);
    // write SECRET
    hasher.update(&self.secret);
    // Return hash
    hasher.finalize().as_slice().to_owned()
  }
  /// Create token
  /// with the given secret
  pub fn create_token(&self) -> String {
    let base = Uuid::new_v4().as_bytes().to_owned();
    let res = base
      .iter()
      .chain(self.create_hash(&base).iter())
      .map(|i| *i)
      .collect::<Vec<u8>>();
    base64::encode(&res)
  }
  /// Verify a given token &str
  /// with the given secret
  pub fn verify(&self, token: &str) -> bool {
    if let Ok(token) = base64::decode(token) {
      if token.len() <= 16 {
        return false;
      }
      let base = &token[0..16];
      let hash = &token[16..];
      return hash == self.create_hash(base);
    }
    false
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[test]
  fn test_hasher() {
    let hash = Hasher::init("hello".as_bytes().to_owned());
    let token = hash.create_token();

    assert_eq!(hash.verify(&token), true);
    assert_eq!(hash.verify("d1cc6a1383a74719a31525e7b0ea6d6cc8430cc9c9cfe7612f5832a47c4a557716f688dbe21d016f9ba9c8e4d6488f0c"), false);
    assert_eq!(hash.verify("12hello"), false);
  }

  #[bench]
  fn bench_verify_token(b: &mut Bencher) {
    let hasher = Hasher::init("hello".as_bytes().to_owned());
    b.iter(|| {
      let token = hasher.create_token();
      let result = hasher.verify(&token);
      assert_eq!(result, true);
    });
  }

  #[bench]
  fn bench_verify_token_long_secret(b: &mut Bencher) {
    let hasher = Hasher::init(Uuid::new_v4().as_bytes().to_owned().into());
    b.iter(|| {
      let token = hasher.create_token();
      let result = hasher.verify(&token);
      assert_eq!(result, true);
    });
  }

  #[bench]
  fn bench_create_token(b: &mut Bencher) {
    let hasher = Hasher::init("hello".as_bytes().to_owned());
    b.iter(|| {
      hasher.create_token();
    });
  }

  #[bench]
  fn bench_create_token_long_secret(b: &mut Bencher) {
    let hasher = Hasher::init(Uuid::new_v4().as_bytes().to_owned().into());
    b.iter(|| {
      hasher.create_token();
    });
  }
}
