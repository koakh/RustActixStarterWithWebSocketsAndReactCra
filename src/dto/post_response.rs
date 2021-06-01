use serde::{Serialize};

#[derive(Serialize, Debug)]
pub struct PostResponse {
  pub message: String,  
}
