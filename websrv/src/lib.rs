//!
#[cfg(all(feature = "drain", feature = "some"))]
compile_error!("feature \"drain\" and feature \"some\" can not be enable at same time");

#[cfg(not(any(feature = "drain", feature = "some")))]
compile_error!("feature \"drain\" and feature \"some\" must be enable someone");

mod cmd;
mod ierror;
mod server;
mod threadpool;

pub use ierror::IError;
pub use server::Server;
pub use threadpool::ThreadPool;
