use anyhow::Result;
use sqlx::migrate::MigrateDatabase;
use sqlx::sqlite::{Sqlite, SqlitePool, SqlitePoolOptions};
use tokio::runtime::Runtime;

use crate::models::ToDoItem;

pub struct SQLRepo {
    pool: SqlitePool,
    rt: Runtime,
}

impl SQLRepo {
    pub fn new(db_url: &str) -> Result<Self> {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
        if !rt.block_on(Sqlite::database_exists(db_url)).unwrap_or(false) {
            println!("Creating database: {}", db_url);
            rt.block_on(Sqlite::create_database(db_url))?;
        }

        let pool = rt.block_on(SqlitePoolOptions::new().max_connections(5).connect(db_url))?;

        let new_repo = Self {
            pool,
            rt,
        };
        Ok(new_repo)
    }

    pub fn add(&self, text: &str) -> Result<ToDoItem> {
        let sql = "INSERT INTO todos (text, completed) values ($1, $2);";
        let query = sqlx::query(sql)
            .bind(text)
            .bind(false)
            .execute(&self.pool);
        let new_id = self.rt.block_on(query)?.last_insert_rowid();

        Ok(ToDoItem {
            id: Some(new_id),
            text: text.to_string(),
            completed: false,
        })
    }

    pub fn get(&self, id: i64) -> Result<Option<ToDoItem>> {
        let sql = "SELECT id, text, completed from todos where id = $1";
        let query = sqlx::query_as::<_, ToDoItem>(sql).bind(id).fetch_optional(&self.pool);
        let item = self.rt.block_on(query)?;
        Ok(item)
    }
}