pub mod models;
struct App {
    card_provider_ranges: Vec<models::CardProviderRange>,
    transactions: Vec<models::Transaction>,
}

impl App {
    pub fn new() -> Self {
        App {
            card_provider_ranges: Vec::new(),
            transactions: Vec::new()
        }
    }
}

fn main() {
    println!("Hello My RUST VERIFONE CONSOLE APP!");
}
