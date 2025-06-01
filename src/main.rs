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

fn prompt_master_password(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    
    io::stdout().flush()?;
    let mut password = read_password()?;
    
    if password.is_empty() {
        anyhow::bail!("Master password cannot be empty");
    }
    
    // Ensure we have a proper string that will be zeroized
    let result = password.clone();
    password.zeroize();

    Ok(result)
}

fn main()-> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing new password vault...");

            let master_password = prompt_master_password("Enter master password: ")?;
            Vault::create_new(&master_password)?;

            println!("Vault created successfully!");
        }

        Commands::Add { service, password } => {
            println!("Add new password");
            
        }

        Commands::Get { service } => {
            println!("Retrieve password");

        }

        Commands::List => {
            println!("List all services with passwords");

        }

        Commands::Remove { service } => {
            println!("Remove password");

        }

        Commands::Update { service, password } => {
            println!("Update password");

        }
    }

    Ok(())
}
