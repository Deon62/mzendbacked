mod cli;
mod database;
mod errors;
mod handlers;
mod models;
mod services;
mod utils;

use cli::CLI;
use colored::Colorize;
use handlers::account_handler::AccountHandler;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        CLI::print_error(&format!("Application error: {}", e));
    }
}

async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let account_handler = AccountHandler::new().await?;

    loop {
        display_main_menu();
        
        let choice = CLI::get_input("Enter your choice:")?;

        match choice.as_str() {
            "1" => {
                if let Err(e) = account_handler.create_account_interactive().await {
                    CLI::print_error(&format!("Error: {}", e));
                }
                wait_for_enter();
            }
            "2" => {
                if let Err(e) = account_handler.login_interactive().await {
                    CLI::print_error(&format!("Error: {}", e));
                }
                wait_for_enter();
            }
            "3" => {
                if let Err(e) = account_handler.show_stats().await {
                    CLI::print_error(&format!("Error: {}", e));
                }
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
    println!("  2. 🔐 Login to Account");
    println!("  3. 📊 Show Database Stats");
    println!("  4. 🚪 Exit");
    println!();
}

fn wait_for_enter() {
    let _ = CLI::get_input("Press Enter to continue...");
}

fn clear_screen() {
    print!("\x1B[2J\x1B[1;1H");
}
