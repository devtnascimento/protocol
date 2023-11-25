use async_trait::async_trait;
pub use serde::{Deserialize, Serialize};
pub use serde_json;
use std::error::Error;
pub use tokio::io::{AsyncReadExt, AsyncWriteExt};
pub use tokio::net::TcpStream;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub name: String,
    pub last_name: String,
    pub cpf: String,
    pub pix_key: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum Status {
    Ok,
    Error(String),
}

#[async_trait]
pub trait Request {
    type Output: Response;

    async fn send(self, addr: &str, port: &str) -> Result<Self::Output>;
}

#[async_trait]
pub trait Response: Serialize + Sized {
    async fn send(self, mut socket: TcpStream) -> Result<()> {
        let resp = serde_json::to_string(&self)?;
        socket.write_all(resp.as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
pub trait Register {
    async fn send(self, addr: &str, port: &str) -> Result<Status>;
}

pub mod register;
pub mod request;
pub mod response;
