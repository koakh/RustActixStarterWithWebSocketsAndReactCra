use serde::{Deserialize, Serialize};
use std::{
  cell::Cell,
  sync::{Arc, Mutex},
};

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PostStateRequest {
  pub message: String,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
  // Mutex is necessary to mutate safely across threads
  pub server_id: usize,
  // worker, used for every thread/workers
  pub request_count: Cell<usize>,
}

#[derive(Debug)]
pub struct AppStateGlobal {
  // global, used for all workers
  pub counter: Mutex<i32>,
  pub message: Arc<Mutex<String>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppStateResponse {
  pub server_id: usize,
  pub counter: i32,
  pub request_count: usize,
  pub message: String,
}
