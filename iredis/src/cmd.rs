use bytes::Bytes;
use mini_redis::cmd::{Get as G, Set as S};
#[derive(Debug)]
pub enum Command {
    Get(G),
    Set(S),
}

impl Command {
    pub fn from_frame(frame: crate::Frame) -> Result<Command, crate::Error> {
        todo!()
    }
}
