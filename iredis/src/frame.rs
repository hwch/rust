use bytes::{Buf, Bytes};
use memchr::memmem;
use std::num::ParseIntError;
use std::process::id;
use std::str::Utf8Error;
use std::{
    fmt::{Display, Formatter, Result as FmtResult},
    io::Cursor,
};

use crate::frame::Error::Incomplete;
use mini_redis::Frame as MiniFrame;

#[derive(Debug, Clone)]
pub enum Frame {
    Simple(String),
    Error(String),
    Integer(u64),
    Bulk(Bytes),
    Null,
    Array(Vec<Frame>),
}

#[derive(Debug)]
pub enum Error {
    /// Not enough data is available to parse a message
    Incomplete,

    /// Invalid message encoding
    Other(crate::Error),
}

impl Frame {
    pub(crate) fn check(src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        let cmd = src.get_u8();

        match cmd {
            b'*' => {
                let data = Frame::get_line(src)?;

                let num = data.trim_ascii_end().parse::<usize>()?;

                let mut idx = 0_usize;
                while idx < num {
                    if src.has_remaining() {
                        let cmd = src.get_u8();

                        Frame::check_non_array(cmd, src)?;
                    } else {
                        break;
                    }
                    idx += 1;
                }
                if idx != num {
                    return Err(Error::Incomplete);
                }
                Ok(())
            } //Array
            _ => Frame::check_non_array(cmd, src),
        }
    }
    fn check_non_array(cmd: u8, src: &mut Cursor<&[u8]>) -> Result<(), Error> {
        match cmd {
            b'+' | b'-' => {
                let _ = Frame::get_line(src)?;
                Ok(())
            } // Simple|Error
            b':' => {
                let integer = Frame::get_line(src)?;
                if let Err(_) = integer.trim_ascii_end().parse::<i64>() {
                    return Err("Integer非数字字符".into());
                }
                Ok(())
            } // Integer
            b'$' => {
                let integer = Frame::get_line(src)?;
                let Ok(v) = integer.trim_ascii_end().parse::<i64>() else {
                    return Err("Bulk or Null非数字字符".into());
                };
                if v < -1 {
                    return Err("Bulk or Null无效数字".into());
                }
                if v >= 0 {
                    let data = Frame::get_line(src)?;
                    if v as usize + 2 != data.len() {
                        return Err(Incomplete);
                    }
                }
                Ok(())
            } // Bulk or Null
            b'*' => unreachable!("Should not reach here"), //Array
            _ => Err("Unkown frame type".into()),
        }
    }
    fn get_line<'a>(cursor: &mut Cursor<&'a [u8]>) -> Result<&'a str, Error> {
        let position = cursor.position() as usize;
        let data = cursor.get_ref();
        let remaining = &data[position..];
        if let Some(pos) = memmem::find(remaining, b"\r\n") {
            let v = str::from_utf8(&remaining[..pos + 2])?;
            cursor.advance(pos + 2);
            return Ok(v);
        }
        return Err(Error::Incomplete);
    }
    pub(crate) fn parse(src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        let cmd = src.get_u8();
        match cmd {
            b'*' => {
                let mut v = vec![];
                let data = Frame::get_line(src)?;
                let num = data.trim_ascii_end().parse::<usize>()?;

                for _ in 0..num {
                    if src.has_remaining() {
                        let cmd = src.get_u8();
                        v.push(Frame::parse_non_array(cmd, src)?);
                    } else {
                        break;
                    }
                }

                Ok(Frame::Array(v))
            } //Array
            _ => Frame::parse_non_array(cmd, src),
        }
    }
    pub(crate) fn parse_non_array(cmd: u8, src: &mut Cursor<&[u8]>) -> Result<Frame, Error> {
        match cmd {
            cmd @ (b'+' | b'-') => {
                let inner = Frame::get_line(src)?.trim_ascii_end();
                let v = if cmd == b'+' {
                    Frame::Simple(inner.into())
                } else {
                    Frame::Error(inner.into())
                };
                Ok(v)
            } // Simple|Error
            b':' => {
                let data = Frame::get_line(src)?;
                let v = Frame::Integer(data.trim_ascii_end().parse::<u64>()?);

                Ok(v)
            } // Integer
            b'$' => {
                let data = Frame::get_line(src)?;
                let Ok(var_len) = data.trim_ascii_end().parse::<i64>() else {
                    return Err("Bulk or Null非数字字符".into());
                };
                if var_len < -1 {
                    return Err("Bulk or Null无效数字".into());
                }

                let res = if var_len >= 0 {
                    let data = Frame::get_line(src)?;
                    if var_len as usize + 2 != data.len() {
                        return Err(Incomplete);
                    }
                    let v = data.trim_ascii_end();

                    Frame::Bulk(Bytes::copy_from_slice(v.as_bytes()))
                } else {
                    Frame::Null
                };
                Ok(res)
            } // Bulk or Null
            b'*' => unreachable!("Should not reach here"), //Array
            _ => Err("Unkown frame type".into()),
        }
    }
    /// Converts the frame to an "unexpected frame" error
    pub(crate) fn to_error(&self) -> crate::Error {
        format!("unexpected frame: {}", self).into()
    }
}

impl Display for Frame {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Frame::Simple(s) => s.fmt(f),
            Frame::Array(a) => {
                for (i, v) in a.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    v.fmt(f)?;
                }
                Ok(())
            }
            Frame::Bulk(b) => match std::str::from_utf8(&b) {
                Ok(v) => v.fmt(f),
                Err(_) => write!(f, "{:?}", b),
            },
            Frame::Error(e) => e.fmt(f),
            Frame::Integer(i) => i.fmt(f),
            Frame::Null => write!(f, "(nil)"),
        }
    }
}

impl From<MiniFrame> for Frame {
    fn from(value: MiniFrame) -> Self {
        match value {
            MiniFrame::Array(v) => {
                let mut vv = vec![];
                for _v in v {
                    vv.push(_v.into());
                }
                Frame::Array(vv)
            }
            MiniFrame::Bulk(v) => Frame::Bulk(v),
            MiniFrame::Error(v) => Frame::Error(v),
            MiniFrame::Null => Frame::Null,
            MiniFrame::Integer(v) => Frame::Integer(v),
            MiniFrame::Simple(v) => Frame::Simple(v),
        }
    }
}

impl From<&str> for Error {
    fn from(value: &str) -> Self {
        value.to_string().into()
    }
}

impl From<Utf8Error> for Error {
    fn from(value: Utf8Error) -> Self {
        value.to_string().into()
    }
}
impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        value.to_string().into()
    }
}
impl From<String> for Error {
    fn from(value: String) -> Self {
        value.into()
    }
}

impl std::error::Error for Error {}
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            Error::Incomplete => write!(f, "Incomplete"),
            Error::Other(e) => e.fmt(f),
        }
    }
}
