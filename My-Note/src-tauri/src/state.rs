



use sqlx::{sqlite::{SqlitePool}};
use std::sync::Mutex;

#[derive(Default)]
pub struct KeyStorage {
    pub key_store: Option<String>,
}

#[derive(Debug)]
pub struct PoolState {
  #[allow(dead_code)]
  pub pool:SqlitePool
}