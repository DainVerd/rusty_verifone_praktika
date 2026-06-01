use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::models::{CardProviderRange, Transaction};
use std::fs::OpenOptions;
use std::io::Write;

pub struct App {
    pub card_provider_ranges: Vec<CardProviderRange>,
    pub transactions: Vec<Transaction>,
}

impl App {
    pub fn new() -> Self {
        App {
            card_provider_ranges: Vec::new(),
            transactions: Vec::new()
        }
    }
    pub fn load_providers(&mut self, file_path: &str) -> Result<(), std::io::Error> {
        let delimiter = ';';
        if file_path.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "File name can not be empty string",
            ));
        }

        // open file if file not found
        // rethrow exception
        let file = File::open(file_path)?;

        // to read from file by line
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?; // rethrow exception if line was not found

            let tokens: Vec<&str> = line.split(delimiter).collect();

            if tokens.len() < 3 {
                continue;
            }

            self.card_provider_ranges.push(
                CardProviderRange {
                    range_start: tokens[0].to_string(),
                    range_end: tokens[1].to_string(),
                    provider_name: tokens[2].to_string()
                }
            );
        }

        Ok(())
    }

    pub fn find_provider(&self, card_number: &str) -> Option<String> {
        if card_number.is_empty() {
            return None;
        }

        for provider in self.card_provider_ranges.iter() {

            if provider.range_start.as_str() <= card_number && provider.range_end.as_str() >= card_number {
                return Some(provider.provider_name.clone())
            }
        }

        None
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        self.transactions.push(transaction);
    }

    pub fn save_transactions(&self) -> Result<(), std::io::Error> {

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open("transactions.txt")?;



        for tx in &self.transactions {
            let euros = tx.sum / 100;
            let cents = tx.sum % 100;
            writeln!(
                file,
                "{};{};{}.{:02};",
                tx.card_number,
                tx.card_provider,
                euros,
                cents
            )?;
        }

        Ok(())
    }
}