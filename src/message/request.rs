use super::*;
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub bank_name: String,
    pub from_user: User,
    pub to_user: User,
    pub amount: f64,
}

#[async_trait]
impl Request for Transaction {
    type Output = response::Transaction;

    async fn send(self, addr: &str, port: &str) -> Result<response::Transaction> {
        let full_address = format!("{}:{}", addr, port);

        let request = serde_json::to_string(&self)?;

        let mut stream = TcpStream::connect(full_address).await?;
        stream.write_all(request.as_bytes()).await?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        let resp = String::from_utf8_lossy(&buffer).to_string();
        let transaction: response::Transaction = serde_json::from_str(&resp)?;

        Ok(transaction)
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pix {
    pub key: String,
}

#[async_trait]
impl Request for Pix {
    type Output = response::Pix;

    async fn send(self, addr: &str, port: &str) -> Result<response::Pix> {
        let full_address = format!("{}:{}", addr, port);

        let request = serde_json::to_string(&self)?;

        let mut stream = TcpStream::connect(full_address).await?;
        stream.write_all(request.as_bytes()).await?;

        let mut buffer = Vec::new();
        stream.read_to_end(&mut buffer).await?;

        let resp = String::from_utf8_lossy(&buffer).to_string();
        let pix: response::Pix = serde_json::from_str(&resp)?;

        Ok(pix)
    }
}
