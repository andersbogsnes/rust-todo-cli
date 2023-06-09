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
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;
        if !rt
            .block_on(Sqlite::database_exists(db_url))
            .unwrap_or(false)
        {
            println!("Creating database: {}", db_url);
            rt.block_on(Sqlite::create_database(db_url))?;
        }

        let pool = rt.block_on(SqlitePoolOptions::new().max_connections(5).connect(db_url))?;

        rt.block_on(sqlx::migrate!("./migrations").run(&pool))?;
        let new_repo = Self { pool, rt };
        Ok(new_repo)
    }

    pub fn add(&self, text: &str) -> Result<ToDoItem> {
        let sql = "INSERT INTO todos (text, completed) values ($1, $2);";
        let query = sqlx::query(sql).bind(text).bind(false).execute(&self.pool);
        let new_id = self.rt.block_on(query)?.last_insert_rowid();

        Ok(ToDoItem {
            id: Some(new_id),
            text: text.to_string(),
            completed: false,
        })
    }

    pub fn get(&self, id: i64) -> Result<Option<ToDoItem>> {
        let sql = "SELECT id, text, completed from todos where id = $1";
        let query = sqlx::query_as::<_, ToDoItem>(sql)
            .bind(id)
            .fetch_optional(&self.pool);
        Ok(self.rt.block_on(query)?)
    }

    pub fn get_all(&self) -> Result<Vec<ToDoItem>> {
        let sql = "SELECT id, text, completed from todos";
        let query = sqlx::query_as::<_, ToDoItem>(sql).fetch_all(&self.pool);
        Ok(self.rt.block_on(query)?)
    }

    pub fn delete(&self, id: i64) -> Result<()> {
        let sql = "DELETE FROM todos where id = $1";
        let query = sqlx::query(sql).bind(id).execute(&self.pool);
        match self.rt.block_on(query)?.rows_affected() {
            1 => Ok(()),
            0 => Err(anyhow::anyhow!("No rows found")),
            _ => Err(anyhow::anyhow!("Multiple rows deleted")),
        }
    }

    pub fn complete(&self, id: i64) -> Result<()> {
        let sql = "UPDATE todos set completed = true where id = $1";
        let query = sqlx::query(sql).bind(id).execute(&self.pool);
        match self.rt.block_on(query)?.rows_affected() {
            1 => Ok(()),
            0 => Err(anyhow::anyhow!("No rows found")),
            _ => Err(anyhow::anyhow!("Multiple rows updated"))
        }
    }
}
