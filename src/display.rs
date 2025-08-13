use prettytable::{Table, row};
use std::collections::HashMap;


pub fn print_table(prices: &HashMap<String,f64>){
    let mut table = Table::new();
    table.add_row(row!["Exchange", "Price(USDT)"]);
    for (exchange, price) in prices {
        table.add_row(row![exchange, format!("{:.2}", price)]);
    }
    table.printstd();
}