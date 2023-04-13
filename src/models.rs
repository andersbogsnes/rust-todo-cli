use std::fmt::{Display, Formatter};

use colored::Colorize;
use sqlx::FromRow;
use tabled::{Table, Tabled};
use tabled::settings::{Format, Modify, Panel, Style, Alignment};
use tabled::settings::object::{FirstColumn, FirstRow};
use tabled::settings::style::BorderSpanCorrection;

#[derive(Debug, Default, FromRow, Tabled, Clone)]
#[tabled(rename_all = "UPPERCASE")]
pub struct ToDoItem {
    #[tabled(display_with = "display_option")]
    pub id: Option<i64>,
    pub text: String,
    pub completed: bool,
}

fn display_option(o: &Option<i64>) -> String {
    match o {
        Some(id) => id.to_string(),
        None => " ".to_string()
    }
}

impl Display for ToDoItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", tableize(vec![self.clone()]))
    }
}

pub fn tableize(items: Vec<ToDoItem>) -> String {
    let mut table = Table::new(items);
    table
        .with(Panel::header("Todo List"))
        .with(Modify::new((0, 0)).with(Alignment::center()))
        .with(
            Modify::new(FirstRow)
                .with(Format::content(|s| s.green().to_string())))
        .with(Modify::new(FirstColumn)
            .with(Format::content(|s| s.green().to_string())))
        .with(Style::modern())
        .with(BorderSpanCorrection)
        .to_string()
}