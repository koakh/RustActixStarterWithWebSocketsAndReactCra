use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct PostInput {
  pub message: String,
}
