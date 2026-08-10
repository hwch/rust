use mini_redis::cmd::{Get as G, Set as S, Unknown};

use crate::Frame;
#[derive(Debug)]
pub enum Command {
    Get(G),
    Set(S),
    Unknown(Unknown),
}

impl Command {
    pub fn from_frame(frame: crate::Frame) -> Result<Command, crate::Error> {
        let arr = match frame {
            crate::Frame::Array(arr) => {
                if arr.is_empty() {
                    return Err("Empty frame".into());
                }
                arr
            }
            _ => return Err("Unsupported frame".into()),
        };
        //肯定不会panic
        match arr.get(0).unwrap() {
            Frame::Bulk(v) => {
                if v.eq_ignore_ascii_case(b"get") {
                    if arr.len() != 2 {
                        return Err("Incomplete `get` command".into());
                    }
                    Ok(Command::Get(G::new(arr[1].clone())))
                } else if v.eq_ignore_ascii_case(b"set") {
                    if arr.len() != 3 {
                        return Err("Incomplete `set` command".into());
                    }
                    if let Frame::Bulk(val) = &arr[2] {
                        Ok(Command::Set(S::new(arr[1].clone(), val.clone(), None)))
                    } else {
                        return Err(format!("Invalid `set` value: {}", &arr[2]).into());
                    }
                } else {
                    return Err("Unsupported command".into());
                }
            }
            _ => return Err("Unsupported frame".into()),
        }
    }
}
