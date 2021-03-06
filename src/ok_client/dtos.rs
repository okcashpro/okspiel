use serde_json::Value;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Walletlocked {
    Locked,
    Unlocked,
    Uncrypted,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Request {
    pub method: String,
    pub params: Option<Value>,
    pub id: Value,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ResponseError {
    pub code: i32,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StakeInfo {
    pub enabled: bool,
    pub staking: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WalletInfo {
    pub walletversion: f32,
    pub balance: f32,
    txcount: f32,
    keypoololdest: f32,
    keypoolsize: f32,
    pub unlocked_until: Option<f32>,
    pub walletlocked: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Account {
    pub address: String,
    pub account: String,
    pub amount: f32,
    pub confirmations: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Transaction {
    pub account: String,
    pub address: String,
    pub category: String,
    pub amount: f32,
    pub confirmations: i32,
    pub blockhash: String,
    pub txid: String,
    pub timereceived: i64,
}

#[derive(Debug, Deserialize)]
pub struct NodeResponse<S> {
    pub result: S,
    pub error: Option<ResponseError>,
}

impl From<(String, Option<Value>, Value)> for Request {
    fn from(rq_options: (String, Option<Value>, Value)) -> Self {
        let (method, params, id) = rq_options;

        Self { method, params, id }
    }
}
