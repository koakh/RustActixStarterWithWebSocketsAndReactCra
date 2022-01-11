use serde::{Serialize};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostWsEchoResponse {
  pub message: String,  
}
