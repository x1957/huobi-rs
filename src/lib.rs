mod types;

use types::*;

type Result<T> = std::result::Result<T, reqwest::Error>;

pub trait Function {
    fn symbols(&self) -> Result<ApiResponse<Symbols>>;
}

pub struct Huobi {
    key: String,
    secret: String,
    uri: &'static str,
    client: reqwest::Client,
}

impl Huobi {
    pub fn new(key: &str, secret: &str, host: &'static str) -> Huobi {
        Huobi {
            key: key.to_string(),
            secret: secret.to_string(),
            uri: host,
            client: reqwest::Client::new(),
        }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_symbols() {
        let huobi = Huobi::new("", "", "https://api.huobi.br.com/");
        let symbol = huobi.symbols().unwrap();
        assert_eq!(symbol.status, "ok");
    }
}
