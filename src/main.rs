use std::fs;

use anyhow::{anyhow, Result};
// TODO: list items
// TODO: delete items
// TODO: mark complete
use clap::{Parser, Subcommand};
use directories::ProjectDirs;

use crate::repo::SQLRepo;

mod repo;
mod models;

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: SubCommand,
}

#[derive(Subcommand, Debug)]
enum SubCommand {
    Add {
        text: String,
    },
    Get { id: i64 },
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


fn main() -> anyhow::Result<()> {
    let args = Cli::parse();
    let repo = init_db()?;
    match args.command {
        SubCommand::Add { text: test } => {
            let new_item = repo.add(&test)?;
            println!("Added item {:?}", new_item);
            Ok(())
        }
        SubCommand::Get { id: item_id } => {
            let item = repo.get(item_id)?;
            if let Some(item) = item {
                println!("Item: {:?}", item)
            } else {
                println!("No item found")
            }
            Ok(())
        }
    }
}
