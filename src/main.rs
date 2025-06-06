mod cli;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;

use crate::cli::CLI;
use crate::handlers::account_handler::AccountHandler;
use colored::Colorize;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let account_handler = AccountHandler::new();

    loop {
        display_main_menu();
        
        let choice = CLI::get_input("Enter your choice:")?;
        
        match choice.as_str() {
            "1" => {
                if let Err(e) = account_handler.create_account_interactive() {
                    CLI::print_error(&format!("Error: {}", e));
                }
                wait_for_enter();
            }
            "2" => {
                CLI::print_info("Stellar wallet connection feature coming soon!");
                wait_for_enter();
            }
            "3" => {
                CLI::print_info("Login feature coming soon!");
                wait_for_enter();
            }
            "4" => {
                CLI::print_info("👋 Thank you for using Stellar Wallet! Goodbye!");
                break;
            }
            _ => {
                CLI::print_error("Invalid choice. Please try again.");
                wait_for_enter();
            }
        }
    }

    Ok(())
}

fn display_main_menu() {
    clear_screen();
    println!("{}", "=".repeat(60).bright_blue());
    println!("{}", "           🌟 STELLAR WALLET BACKEND 🌟           ".bright_yellow().bold());
    println!("{}", "=".repeat(60).bright_blue());
    println!();
    println!("{}", "Main Menu:".cyan().bold());
    println!("  1. 📝 Create New Account");
    println!("  2. 🔗 Connect Stellar Wallet (Coming Soon)");
    println!("  3. 🔐 Login (Coming Soon)");
    println!("  4. 🚪 Exit");
    println!();
}

fn wait_for_enter() {
    println!();
    let _ = CLI::get_input("Press Enter to continue...");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
