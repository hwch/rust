//!
#[cfg(all(feature = "vec", feature = "bytes"))]
compile_error!("feature `vec` and feature `bytes` can not be enable at same time");

#[cfg(not(any(feature = "vec", feature = "bytes")))]
compile_error!("feature `vec` and feature `bytes` must be enable someone");

pub mod cmd;
pub mod conn;
pub mod frame;
pub mod types;

pub use cmd::Command;
pub use conn::Connection;
pub use frame::Frame;
pub use types::Error;
