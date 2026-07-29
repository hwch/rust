// use crate::frame::Frame;
use crate::types::Result;
use bytes::{Buf, BytesMut};
use mini_redis::Frame;
use std::io::Cursor;
use tokio::io::{AsyncReadExt,AsyncWriteExt,BufWriter};
use tokio::net::TcpStream;

#[cfg(feature = "vec")]
struct Connection {
    stream: TcpStream,
    buffer: Vec<u8>,
    cursor: usize,
}

#[cfg(feature = "bytes")]
struct Connection {
    stream: BufWriter<TcpStream>,
    buffer: BytesMut,
}

impl Connection {
    const SEND_SIZE: usize = 4096;
    #[cfg(feature = "bytes")]
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
            buffer: BytesMut::with_capacity(4096),
        }
    }
    #[cfg(feature = "vec")]
    pub fn new(stream: TcpStream) -> Self {
        Self {
            stream: BufWriter::new(stream),
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
    pub async fn write_frame(&mut self, frame: &Frame) -> Result<()> {
        match frame {
            Frame::Array(val) => {
                // Encode the frame type prefix. For an array, it is `*`.
                self.stream.write_u8(b'*').await?;

                // Encode the length of the array.
                self.write_decimal(val.len() as u64).await?;

                // Iterate and encode each entry in the array.
                for entry in &*val {
                    self.write_frame_non_array(entry).await?;
                }
            }
            _ => self.write_frame_non_array(frame).await?,
            
        }
        self.flush().await?;
        Ok(())
    }
    async fn write_frame_non_array(&mut self, frame: &Frame) -> Result<()> {
        
        match frame {
            Frame::Simple(v) => {
                self.stream.write_u8(b'+').await?;
                self.stream.write_all(v.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Error(val) => {
                self.stream.write_u8(b'-').await?;
                self.stream.write_all(val.as_bytes()).await?;
                self.stream.write_all(b"\r\n").await?;
            }
            Frame::Integer(val) => {
                self.stream.write_u8(b':').await?;
                self.write_decimal(*val).await?;
            }
            Frame::Null => {
                self.stream.write_all(b"$-1\r\n").await?;
            }
            Frame::Bulk(val) => {
                let len = val.len();

                self.stream.write_u8(b'$').await?;

                self.write_decimal(len as u64).await?;
                self.stream.write_all(val).await?;
                self.stream.write_all(b"\r\n").await?;
                self.stream.flush().await?; //立马清缓存
            }
            Frame::Array(_) => unreachable!("Should not reach here"),
            }
            Ok(())
        }
    }
    async fn flush(&mut self) -> Result<()> {
        if self.stream.buffer().len() > Connection::SEND_SIZE {
            self.stream.flush().await?;
        }
        Ok(())
    }

    async fn write_decimal(&mut self, v: u64) -> Result<()> {
        use std::io::Write;
        let buf = [0u8; 12];
        let mut buf = Cursor::new(buf);
        let _ = write!(&mut buf, "{}", v);

        let pos = buf.position() as usize;
        self.stream.write_all(&buf.get_ref()[..pos]).await?;
        self.stream.write_all(b"\r\n").await?;
        Ok(())
    }
    fn parse_frame(&mut self) -> Result<Option<Frame>> {
        if self.buffer.len() == 0 {
            return Ok(None);
        }
        
        let mut cursor = Cursor::new(&self.buffer[..]);
        match Frame::check(&mut cursor) {
            //会修改position
            Ok(_) => {
                let len = cursor.position() as usize;
                cursor.set_position(0);
                let frame = Frame::parse(&mut cursor)?;
                self.buffer.advance(len);

                return Ok(Some(frame));
            }
            Err(mini_redis::frame::Error::Incomplete) => return Ok(None),
            Err(e) => return Err(e.into()),
        }
    }
}
