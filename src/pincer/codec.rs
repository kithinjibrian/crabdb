use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

use crate::pincer::message::Message;

pub async fn read_frame(stream: &mut TcpStream) -> Result<Message, ReadFrameError> {
    let mut len_buf = [0u8; 4];
    if let Err(e) = stream.read_exact(&mut len_buf).await {
        return Err(ReadFrameError::Io(e));
    }

    let len = u32::from_be_bytes(len_buf) as usize;

    if len > 16 * 1024 * 1024 {
        return Err(ReadFrameError::FrameTooLarge(len));
    }

    let mut payload = vec![0u8; len];
    if let Err(e) = stream.read_exact(&mut payload).await {
        return Err(ReadFrameError::Io(e));
    }

    let msg: Message = serde_json::from_slice(&payload).map_err(ReadFrameError::Json)?;

    Ok(msg)
}

pub async fn write_frame(stream: &mut TcpStream, msg: &Message) -> Result<(), WriteFrameError> {
    let body = serde_json::to_vec(msg).map_err(WriteFrameError::Json)?;

    let len = (body.len() as u32).to_be_bytes();

    stream.write_all(&len).await.map_err(WriteFrameError::Io)?;

    stream.write_all(&body).await.map_err(WriteFrameError::Io)?;

    Ok(())
}

#[derive(Debug)]
pub enum ReadFrameError {
    Io(std::io::Error),
    Json(serde_json::Error),
    FrameTooLarge(usize),
}

#[derive(Debug)]
pub enum WriteFrameError {
    Io(std::io::Error),
    Json(serde_json::Error),
}
