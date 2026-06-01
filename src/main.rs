pub mod models;
mod app;
use app::App;
use std::io;
use std::io::Write;
use crate::models::Transaction;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut app = App::new();

    println!("loading data for providers...");
    app.load_providers("resources/file.txt")?;
    println!("data is loaded.\n");

    loop {
        print!("enter card number or 'q' for exit: ");
        io::stdout().flush()?; // clear buffer

        let mut card_number = String::new();
        io::stdin()
            .read_line(&mut card_number)?;

        let card_number = card_number.trim();

        // escape check
        if card_number == "q" {
            println!("Quit the program, By!");
            break;
        }

        let card_provider = match app.find_provider(&card_number) {
            Some(provider) => provider,
            None => {
                println!("Error: Did not find provider of provided card.");
                continue;
            }
        };
        println!("enter transaction sum like so xx.yy or press 'q' for exit: ");
        let mut input_price = String::new();
        io::stdin()
            .read_line(&mut input_price)?;

        let input_price = input_price.trim();
        if input_price == "q" {
            println!("Quit the program, By!");
            break;
        }

        let price_f64: f64 = input_price.parse().unwrap_or(0.0);

        let cents_f64 = (price_f64 * 100.0).round();

        let cents_u64 = cents_f64 as u64;

        app.add_transaction(Transaction {
            card_number: card_number.to_string().clone(),
            card_provider: card_provider.clone(),
            sum: cents_u64,
        });
        println!("Print transactions into file? press 'Y' if you want or enter to continue.");
        let mut print_choice = String::new();
        io::stdin()
            .read_line(&mut print_choice)?;

        let print_choice = print_choice.trim();
        if print_choice == "Y" {
            println!("saving all transactions into a file");

            app.save_transactions()?;
        }
    }

    Ok(())
}
