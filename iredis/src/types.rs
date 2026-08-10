use bytes::Bytes;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

/// A specialized `Result` type for mini-redis operations.
///
/// This is defined as a convenience.
pub type Result<T> = std::result::Result<T, Error>;

pub type Db = Arc<Mutex<HashMap<String, Bytes>>>;
