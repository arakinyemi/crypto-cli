use crate::errors::AppError;
use crate::config::Config;
use reqwest::blocking::Client;
use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::Deserialize;
use std::collections::HashMap;

const TARGET_EXCHANGES: &[&str] = &["binance", "kraken", "kucoin", "coinbase-exchange", "bitfinex"];

#[derive(Debug, Deserialize)]
struct Response{
    data: MarketPairsData
}

#[derive(Debug, Deserialize)]
struct MarketPairsData{
    marketpair: Vec<MarketPairItem>
}

#[derive(Debug, Deserialize)]
struct MarketPairItem{
    exchange_slug: String,
    quote: HashMap<String, QuoteData>
}

#[derive(Debug, Deserialize)]
struct QuoteData {
    price: f64
}

pub fn fetch_btc_prices(cfg: &Config) -> Result<HashMap<String, f64>, AppError> {
    
    let url = "https://sandbox-api.coinmarketcap.com/v2/cryptocurrency/market-pairs/latest";

    let client = Client::builder()
        .build()?;
    let resp = client.get(url)
        .header("X-CMC_PRO_API_KEY", &cfg.api_key)
        .header(ACCEPT, "application/json")
        .header(CONTENT_TYPE, "application/json")
        .query(&[("symbol", "BTC"), ("convert", "USD"), ("limit", "500")])
        .send()?
        .error_for_status()?
        .json::<Response>();

    let mut prices = HashMap::new();
    let response = match resp {
        Ok(r) => r,
        Err(e) => return Err(AppError::NoData(format!("Failed to parse response: {}\n", e))),
    };
    for pair in response.data.marketpair {
        if TARGET_EXCHANGES.contains(&pair.exchange_slug.as_str()){
            if let Some(q) = pair.quote.get("USDT") {
                let name = match pair.exchange_slug.as_str(){
                    "binance" => "Binance",
                    "kraken" => "Kraken",
                    "kucoin"  => "KuCoin",
                    "coinbase-exchange" => "Coinbase",
                    "bitfinex" => "Bitfinex",
                    other => other,
                };
                prices.insert(name.to_string(), q.price);
            }
        }
    }
     if prices.is_empty() {
        return Err(AppError::NoData("No BTC/USDT pairs found.".into()));
    }

    Ok(prices)

}
