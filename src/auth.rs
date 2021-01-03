use chrono::prelude::*;
use packman::*;
use serde::{Deserialize, Serialize}

#[derive(Deserialize, Serialize, Clone)]
pub struct AuthObject {
  // Unique ID
  id: u32,
  // Related UID
  uid: u32,
  // Access token
  token: String,
  // Last query DateTime<Utc>
  last_used: Option<DateTime<Utc>>,
  // Total query count using this count
  query_count: usize,
  // Client first use user_agent
  // for fingerprint check
  created_user_agent: String,
  // Client IP who created
  created_ip: String,
  // Token creation DateTime<Utc>
  created_at: DateTime<Utc>,
  // UID who created this Token
  created_by: u32,
}

impl Default for AuthObject {
    fn default() -> Self {
        Self {
            id: 0,
            uid: 0,
            token: "".into(),
            last_used: None,
            query_count: 0,
            created_user_agent: "".into(),
            created_ip: "".into(),
            created_at: Utc::now(),
            created_by: 0,
          
        }
    }
}

impl VecPackMember for AuthObject {
  type Out = u32;

  fn get_id(&self) -> &Self::Out {
    &self.id
  }
}