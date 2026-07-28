use crate::frame::Frame;
use crate::types::Result;
use bytes::BytesMut;
use mini_redis::Frame as RedisFrame;
use std::io::Cursor;
use tokio::io::AsyncReadExt;
use tokio::net::TcpStream;

#[cfg(feature = "vec")]
struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

#[cfg(feature = "bytes")]
struct Connection {
    stream: TcpStream,
    buffer: BytesMut,
    cursor: usize,
}

impl Connection {
    #[cfg(feature = "bytes")]
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: BytesMut::with_capacity(4096),
            cursor: 0,
        }
    }
    #[cfg(feature = "vec")]
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream,
            buffer: vec![0; 4096],
            cursor: 0,
        }
    }
    /// 从连接读取一个帧
    ///
    /// 如果遇到EOF，则返回 None
    #[cfg(feature = "bytes")]
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // 尝试从缓冲区的数据中解析出一个数据帧，
            // 只有当数据足够被解析时，才返回对应的帧
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }

            // 如果缓冲区中的数据还不足以被解析为一个数据帧，
            // 那么我们需要从 socket 中读取更多的数据
            //
            // 读取成功时，会返回读取到的字节数，0 代表着读到了数据流的末尾
            if 0 == self.stream.read_buf(&mut self.buffer).await? {
                // 代码能执行到这里，说明了对端关闭了连接，
                // 需要看看缓冲区是否还有数据，若没有数据，说明所有数据成功被处理，
                // 若还有数据，说明对端在发送帧的过程中断开了连接，导致只发送了部分数据
                if self.buffer.is_empty() {
                    return Ok(None);
                } else {
                    return Err("force to close by peer".into());
                }
            }
        }
    }
    #[cfg(feature = "vec")]
    pub async fn read_frame(&mut self) -> Result<Option<Frame>> {
        loop {
            // 尝试从缓冲区的数据中解析出一个数据帧，
            // 只有当数据足够被解析时，才返回对应的帧
            if let Some(frame) = self.parse_frame()? {
                return Ok(Some(frame));
            }
            if self.cursor >= self.buffer.len() {
                self.cursor *= 2;
                self.buffer.resize(self.cursor, 0);
            }
            let n = self.stream.read(&mut self.buffer[self.cursor..]).await?;
            if n == 0 {
                // 代码能执行到这里，说明了对端关闭了连接，
                // 需要看看缓冲区是否还有数据，若没有数据，说明所有数据成功被处理，
                // 若还有数据，说明对端在发送帧的过程中断开了连接，导致只发送了部分数据
                if self.cursor == 0 {
                    return Ok(None);
                } else {
                    return Err("force to close by peer".into());
                }
            } else {
                self.cursor += n;
            }
        }
    }

    /// 将帧写入到连接中
    pub async fn write_frame(&mut self) -> Result<()> {
        Ok(())
    }

    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        Ok(None)
    }
}
