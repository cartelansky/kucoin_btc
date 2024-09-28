use reqwest;
use serde_json::Value;
use std::error::Error;
use std::fs::File;
use std::io::Write;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let url = "https://api.kucoin.com/api/v1/market/allTickers";
    let response = reqwest::get(url).await?.text().await?;
    let data: Value = serde_json::from_str(&response)?;

    let mut markets: Vec<String> = Vec::new();

    if let Some(ticker_list) = data["data"]["ticker"].as_array() {
        for ticker in ticker_list {
            if let Some(symbol) = ticker["symbol"].as_str() {
                if symbol.ends_with("-BTC") {
                    let base_currency = symbol.split('-').next().unwrap();
                    markets.push(format!("KUCOIN:{}BTC", base_currency));
                }
            }
        }
    }

    markets.sort_by(|a, b| {
        let a_numeric = a.chars().next().unwrap().is_numeric();
        let b_numeric = b.chars().next().unwrap().is_numeric();

        match (a_numeric, b_numeric) {
            (true, true) => b.cmp(a),
            (true, false) => std::cmp::Ordering::Less,
            (false, true) => std::cmp::Ordering::Greater,
            (false, false) => a.cmp(b),
        }
    });

    let mut file = File::create("kucoin_btc_markets.txt")?;
    for market in markets {
        writeln!(file, "{}", market)?;
    }

    println!("İşlem tamamlandı. Veriler 'kucoin_btc_markets.txt' dosyasına kaydedildi.");
    Ok(())
}
