mod signature;
mod types;

use chrono::prelude::Utc;
use log::debug;
use reqwest::Method;
use reqwest::Url;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use types::*;

const URL_HUOBI_PRO: &str = "api.huobi.pro";

type Result<T> = std::result::Result<T, reqwest::Error>;

pub trait Function {
    fn symbols(&self) -> Result<ApiResponse<Symbols>>;
    fn account(&self) -> Result<ApiResponse<Accounts>>;
}

pub struct Huobi {
    key: String,
    secret: String,
    id: u64,
    uri: &'static str,
    client: reqwest::Client,
}

impl Huobi {
    pub fn new(key: &str, secret: &str, id: u64, host: &'static str) -> Huobi {
        let mut default_headers = http::HeaderMap::new();
        default_headers.insert("Content-Type", "application/json".parse().unwrap());
        default_headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 6.1; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.71 Safari/537.36".parse().unwrap());
        Huobi {
            key: key.to_string(),
            secret: secret.to_string(),
            id: id,
            uri: host,
            client: reqwest::Client::builder()
                .tcp_nodelay()
                .default_headers(default_headers)
                .build()
                .unwrap(),
        }
    }

    fn get_body(&self) -> HashMap<String, String> {
        let mut map = HashMap::new();
        map.insert("AccessKeyId".to_string(), self.key.to_string());
        map.insert("SignatureMethod".to_string(), "HmacSHA256".to_string());
        map.insert("SignatureVersion".to_string(), "2".to_string());
        let ts = Utc::now();
        map.insert(
            "Timestamp".to_string(),
            ts.format("%Y-%m-%dT%H:%M:%S").to_string(),
        );
        map
    }

    fn call_api<T: DeserializeOwned>(
        &self,
        method: &str,
        url: &str,
        body: &HashMap<String, String>,
    ) -> Result<ApiResponse<T>> {
        let method = match method {
            "GET" => Method::GET,
            "POST" => Method::POST,
            _ => panic!("unknown http method"),
        };
        let b = match body.len() {
            0 => "".to_string(),
            _ => serde_json::to_string(body).unwrap(),
        };
        let u: Url = url.parse().unwrap();
        let result = self
            .client
            .request(method, u)
            .body(b)
            .send()?
            .json::<ApiResponse<T>>()?;
        Ok(result)
    }
}

impl Function for Huobi {
    fn symbols(&self) -> Result<ApiResponse<Symbols>> {
        let mut url = self.uri.to_string();
        let suffix = "/market/symbols";
        url += &suffix;
        let symbols: ApiResponse<Symbols> = self.client.get(&url).send()?.json()?;
        Ok(symbols)
    }

    fn account(&self) -> Result<ApiResponse<Accounts>> {
        let mut url = self.uri.to_string();
        let suffix = "/v1/account/accounts";
        url += &suffix;
        let body = self.get_body();
        let payload = signature::sign("GET", URL_HUOBI_PRO, suffix, &body, &self.secret);
        url += "?";
        url += &payload;
        debug!("url = {}", url);
        self.call_api("GET", &url, &body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbols() {
        let huobi = Huobi::new("", "", 1957, "https://api.huobi.br.com/");
        let symbol = huobi.symbols().unwrap();
        assert_eq!(symbol.status, "ok");
    }

    #[test]
    fn test_get_accounts() {
        let huobi = Huobi::new("***", "****", 1957, "https://api.huobi.pro");
        let accounts = huobi.account().unwrap();
        assert_eq!(accounts.status, "ok");
    }
}
