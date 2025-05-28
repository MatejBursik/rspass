mod cli;

use cli::{Args, Commands};

use clap::Parser;

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing new password vault...");
            
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
}