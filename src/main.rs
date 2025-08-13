mod api;
mod config;
mod errors;
mod display;

use dotenv;
use config::Config;
use api::fetch_btc_prices;
use display::print_table;
// use errors::AppError;


fn main() {
    dotenv::dotenv().ok();
    match std::env::var("API_KEY") {
        Ok(val) => println!("API_KEY loaded: {}\n", val),
        Err(_) => println!("API_KEY not found in environment"),
    }
    let cfg = match Config::from_env() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Config error: {}", e);
            return;
        }
    };

    let prices = match fetch_btc_prices(&cfg) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };
    print_table(&prices);
}
