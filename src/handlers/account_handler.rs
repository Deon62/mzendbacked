use crate::cli::CLI;
use crate::errors::Result;
use crate::models::user::CreateUserRequest;
use crate::services::user_service::UserService;
use crate::utils::validation::Validator;
use colored::Colorize;

pub struct AccountHandler {
    user_service: UserService,
}

impl AccountHandler {
    pub fn new() -> Self {
        Self {
            user_service: UserService::new(),
        }
    }

    pub fn create_account_interactive(&self) -> Result<()> {
        CLI::print_header();
        CLI::print_info("Let's create your Stellar Wallet account!");
        println!();

        // Get email
        let email = loop {
            let email = CLI::get_input("📧 Enter your email address:")?;
            
            if email.is_empty() {
                CLI::print_error("Email cannot be empty");
                continue;
            }

            match Validator::validate_email(&email) {
                Ok(()) => break email,
                Err(e) => {
                    CLI::print_error(&e.to_string());
                    continue;
                }
            }
        };

        // Get username
        let username = loop {
            let username = CLI::get_input("👤 Choose a username:")?;
            
            if username.is_empty() {
                CLI::print_error("Username cannot be empty");
                continue;
            }

            match Validator::validate_username(&username) {
                Ok(()) => break username,
                Err(e) => {
                    CLI::print_error(&e.to_string());
                    continue;
                }
            }
        };

        // Get password with confirmation
        println!();
        CLI::display_password_requirements();
        
        let password = loop {
            let password = CLI::get_password("🔒 Enter your password:")?;
            
            if password.is_empty() {
                CLI::print_error("Password cannot be empty");
                continue;
            }

            // Validate password strength
            match Validator::validate_password(&password) {
                Ok(()) => {
                    // Confirm password
                    let confirm_password = CLI::get_password("🔒 Confirm your password:")?;
                    
                    if password != confirm_password {
                        CLI::print_error("Passwords do not match. Please try again.");
                        continue;
                    }
                    
                    break password;
                }
                Err(e) => {
                    CLI::print_error(&e.to_string());
                    continue;
                }
            }
        };

        // Display summary and confirm
        println!();
        println!("{}", "Account Summary:".yellow().bold());
        println!("📧 Email: {}", email);
        println!("👤 Username: {}", username);
        println!("🔒 Password: {}", "*".repeat(password.len()));
        println!();

        if !CLI::confirm_action("Do you want to create this account?")? {
            CLI::print_info("Account creation cancelled.");
            return Ok(());
        }

        // Create the account
        let create_request = CreateUserRequest {
            email: email.clone(),
            username: username.clone(),
            password,
        };

        match self.user_service.create_user(create_request) {
            Ok(user) => {
                println!();
                CLI::print_success("🎉 Account created successfully!");
                println!();
                println!("{}", "Account Details:".green().bold());
                println!("🆔 User ID: {}", user.id);
                println!("📧 Email: {}", user.email);
                println!("👤 Username: {}", user.username);
                println!("📅 Created: {}", user.created_at.format("%Y-%m-%d %H:%M:%S UTC"));
                println!("✉️  Verification Status: {}", if user.is_verified { "Verified" } else { "Pending" });
                println!();
                CLI::print_info("Your account has been created! You can now proceed to connect your Stellar wallet.");
            }
            Err(e) => {
                CLI::print_error(&format!("Failed to create account: {}", e));
                return Err(e);
            }
        }

        Ok(())
    }
}
