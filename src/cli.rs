use clap::{Parser, Subcommand};

#[derive(Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    // Initialize a new password vault
    Init,
    
    // Add a new password for a service
    Add {
        service: String, // Service name (github, email, ...)
        #[arg(short, long)]
        password: Option<String>, // Password to store
    },
    
    // Retrieve a password for a service
    Get {
        service: String,
    },
    
    // List all stored service names
    List,
    
    // Remove a password for a service
    Remove {
        service: String,
    },
    
    // Update an existing password for a service
    Update {
        service: String,
        #[arg(short, long)]
        password: Option<String>,
    },
}
