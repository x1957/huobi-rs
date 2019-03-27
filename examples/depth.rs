use huobi::{Function, Huobi};

fn main() {
    let huobi = Huobi::new("", "", 1957, "https://api.huobi.br.com");
    let depth = huobi.depth("btcusdt", "step1").unwrap();
    println!("{:?}", depth);
}
