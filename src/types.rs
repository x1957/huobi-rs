use serde_derive::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub status: String,
    pub ch: String,
    pub ts: Option<u64>,
    pub data: Option<T>,
    pub tick: Option<T>,
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
