use super::*;
#[derive(Debug, Deserialize, Serialize)]
pub struct Transaction {
    pub status: Status,
}

impl Response for Transaction {}

#[derive(Debug, Deserialize, Serialize)]
pub struct Pix {
    pub status: Status,
    pub user: User,
}

impl Response for Pix {}
