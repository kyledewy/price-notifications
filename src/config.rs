use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub currencies: Vec<String>,
    pub priced_in: String,
    pub notify_above: HashMap<String, Decimal>,
    pub notify_below: HashMap<String, Decimal>,
    pub my_number: String,
    pub to_number: String,
    pub account_id: String,
    pub auth_token: String,
}

impl Default for Config {
    fn default() -> Config {
        Config {
            currencies: Vec::new(),
            priced_in: String::from("usd"),
            notify_above: HashMap::new(),
            notify_below: HashMap::new(),
            my_number: String::from(""),
            to_number: String::from(""),
            account_id: String::from(""),
            auth_token: String::from(""),
        }
    }
}

impl std::fmt::Display for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut output = String::new();
        for currency in self.currencies.iter() {
            output = format!("\n{}{}", output, &currency);
        }
        write!(f, "{}", output)
    }
}
