use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub ch: Option<String>,
    pub ts: Option<u64>,
    pub data: Option<T>,
    pub tick: Option<T>,
    #[serde(rename = "err-msg")]
    pub err_msg: Option<String>,
    #[serde(rename = "err-code")]
    pub err_code: Option<String>,
}

pub type Symbols = HashMap<String, Symbol>;

#[derive(Deserialize, Debug)]
pub struct Symbol {
    #[serde(rename = "amount-scale")]
    pub amount_scale: i64,
    #[serde(rename = "price-scale")]
    pub price_scale: i64,
    #[serde(rename = "depth-steps")]
    pub depth_steps: DepthSteps,
}

pub type DepthSteps = HashMap<u32, Vec<f64>>;

pub type Accounts = Vec<Account>;

#[derive(Serialize, Deserialize, Debug)]
pub struct Account {
    pub id: u64,
    #[serde(rename = "type")]
    pub account_type: String,
    pub state: String,
}
