use super::*;
#[derive(Debug, Deserialize, Serialize)]
pub struct Pix {
    pub bank_name: String,
    pub users: Vec<User>,
}

#[async_trait]
impl Register for Pix {
    async fn send(self, addr: &str, port: &str) -> Result<Status> {
        let full_address = format!("{}:{}", addr, port);

        let request = serde_json::to_string(&self)?;

        let mut stream = TcpStream::connect(full_address).await?;
        stream.write_all(request.as_bytes()).await?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        let resp = String::from_utf8_lossy(&buffer).to_string();
        let status: Status = serde_json::from_str(&resp)?;

        Ok(status)
    }
}
