use sqlx::FromRow;

#[derive(Debug, Default, FromRow)]
pub struct ToDoItem {
    pub id: Option<i64>,
    pub text: String,
    pub completed: bool,
}

