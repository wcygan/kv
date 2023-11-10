use client::Client;

use clap::{Parser, Subcommand};


/// Command line arguments for the KV Client
#[derive(Parser)]
#[clap(name = "KV Client", version = "0.1.0", author = "Your Name")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Put a value into the data store
    Put {
        /// The key to store the value under
        key: String,
        /// The value to be stored
        value: String,
    },
    /// Get a value from the data store
    Get {
        /// The key to get the value for
        key: String,
    },
    /// Delete a value from the data store
    Delete {
        /// The key for the value to be deleted
        key: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let mut client = Client::new("http://localhost:7676".to_string()).await?;
    match cli.command {
        Commands::Put { key, value } => {
            match client.put(key, value).await {
                Ok(_) => println!("Value stored"),
                Err(e) => println!("Error: {}", e.to_string()),
            }
        }
        Commands::Get { key } => {
            let response = client.get::<String, String>(key.clone()).await;

            if let Err(e) = response {
                println!("Error: {}", e.to_string());
                return Ok(());
            }

            match response.unwrap() {
                Some(value) => println!("Value: {}", value),
                None => println!("Key '{key}' not found"),
            }
        }
        Commands::Delete { key } => {
            match client.delete(key).await {
                Ok(_) => println!("Value deleted"),
                Err(e) => println!("Error: {}", e.to_string()),
            }
        }
    }

    Ok(())
}
