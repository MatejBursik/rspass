mod cli;
mod encrypt;
mod vault;

use cli::{Args, Commands};
use vault::Vault;

use anyhow::Result;
use clap::Parser;
use rpassword::read_password;
use std::io::{self, Write};
use zeroize::Zeroize;

fn prompt_password(prompt: &str, is_master: bool) -> Result<String> {
    print!("{}", prompt);
    
    io::stdout().flush()?;
    let mut password = read_password()?;
    
    if password.is_empty() {
        if is_master {
            anyhow::bail!("Master password cannot be empty");
        }
        
        anyhow::bail!("Password cannot be empty");
    }
    
    // Ensure we have a proper string that will be zeroized
    let result = password.clone();
    password.zeroize();

    Ok(result)
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing new password vault...");

            let master_password = prompt_password("Enter master password: ", true)?;
            Vault::create_new(&master_password)?;

            println!("Vault created successfully!");
        }

        Commands::Add { service, password } => {
            let master_password = prompt_password("Enter master password: ", true)?;
            let mut vault = Vault::load(&master_password)?;
            
            let password = if let Some(pwd) = password {
                pwd
            } else {
                prompt_password("Enter password to store: ", false)?
            };
            
            vault.add_password(&service, &password)?;
            vault.save(&master_password)?;

            println!("Password added for '{}'", service);
        }

        Commands::Get { service } => {
            let master_password = prompt_password("Enter master password: ", true)?;
            let vault = Vault::load(&master_password)?;
            
            if let Some(password) = vault.get_password(&service) {
                println!("Password for '{}': {}", service, password);
            } else {
                println!("No password found for '{}'", service);
            }
        }

        Commands::List => {
            let master_password = prompt_password("Enter master password: ", true)?;
            let vault = Vault::load(&master_password)?;
            
            let services = vault.list_services();
            if services.is_empty() {
                println!("No passwords stored yet");
            } else {
                println!("Stored services:");
                for service in services {
                    println!(" - {}", service);
                }
            }
        }

        Commands::Remove { service } => {
            let master_password = prompt_password("Enter master password: ", true)?;
            let mut vault = Vault::load(&master_password)?;
            
            if vault.remove_password(&service)? {
                vault.save(&master_password)?;
                println!("Password removed for '{}'", service);
            } else {
                println!("No password found for '{}'", service);
            }
        }

        Commands::Update { service, password } => {
            let master_password = prompt_password("Enter master password: ", true)?;
            let mut vault = Vault::load(&master_password)?;
            
            let password = if let Some(pwd) = password {
                pwd
            } else {
                prompt_password("Enter new password: ", false)?
            };
            
            vault.update_password(&service, &password)?;
            vault.save(&master_password)?;

            println!("Password updated for '{}'", service);
        }
    }

    Ok(())
}
