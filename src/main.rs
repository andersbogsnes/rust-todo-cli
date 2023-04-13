use std::fs;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use directories::ProjectDirs;

use crate::models::tableize;
use crate::repo::SQLRepo;

mod models;
mod repo;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Add { text: String },
    Get { id: i64 },
    List,
    Delete { id: i64 },
    Complete { id: i64 },
}

fn init_db() -> Result<SQLRepo> {
    if let Some(dir) = ProjectDirs::from("", "", "todoer") {
        let data_dir = dir.config_dir();
        fs::create_dir_all(&data_dir)?;
        let db_path = data_dir.join("todo.db");
        let repo = SQLRepo::new(&format!("sqlite://{}", db_path.to_string_lossy()))?;

        Ok(repo)
    } else {
        Err(anyhow!("Error"))
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();
    let repo = init_db()?;

    match args.command {
        SubCommand::Add { text: test } => {
            let new_item = repo.add(&test)?;
            println!("Added item {}", new_item);
            Ok(())
        }
        SubCommand::Get { id } => {
            match repo.get(id)? {
                Some(item) => println!("Item: {}", item),
                None => println!("No item found"),
            }
            Ok(())
        }

        SubCommand::List => {
            let items = repo.get_all()?;
            println!("{}", tableize(items));
            Ok(())
        }

        SubCommand::Delete { id } => {
            repo.delete(id)?;
            Ok(())
        }
        SubCommand::Complete { id } => {
            repo.complete(id)?;
            Ok(())
        }
    }
}
