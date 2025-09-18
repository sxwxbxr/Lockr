use anyhow::Result;
use clap::{Parser, Subcommand};
use secrecy::Secret;
use lockr_core::vault::{Vault, Entry};

#[derive(Parser)]
#[command(name="lockr", version)]
struct Cli {
    #[command(subcommand)]
    cmd: Commands,
}

#[derive(Subcommand)]
enum Commands {
    New { master: String },
    Add { master: String, title: String, username: String, password: String },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Commands::New { master } => {
            let v = Vault::new();
            let ct = v.serialize_encrypted(&Secret::new(master))?;
            println!("salt and nonce sind im JSON header spaeter. hier nur ct len {}", ct.len());
        }
        Commands::Add { master, title, username, password } => {
            let mut v = Vault::new();
            v.add(Entry { title, username, password: Secret::new(password), url: None, notes: None });
            let ct = v.serialize_encrypted(&Secret::new(master))?;
            println!("ct len {}", ct.len());
        }
    }
    Ok(())
}
