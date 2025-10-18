use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    // will implement this at a later point
    // subtasks: Vec<SubTask>,
    pub follow_up_date: Option<NaiveDate>,
    pub last_update: Option<NaiveDate>,
    // update_freq: Option<Frequency>, // Daily - Weekly - Custom
    pub completed: bool,
}

// will implement at later point
// #[derive(FromRow, Debug)]
// struct SubTask {
//     task_id: u32,
//     text: String,
//     completed: bool,
// }

// #[derive(Debug)]
// pub enum Frequency {
//     Daily,
//     Weekly,
//     Custom(NaiveDate),
// }

#[derive(Debug)]
pub struct NewTask {
    title: String,
}

impl NewTask {
    pub fn new(title: &str) -> Self {
        NewTask {
            title: title.to_string(),
        }
    }
    pub fn title(&self) -> &String {
        &self.title
    }
}
