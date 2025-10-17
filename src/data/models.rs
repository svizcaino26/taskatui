use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Task {
    id: i32,
    title: String,
    description: Option<String>,
    // will implement this at a later point
    // subtasks: Vec<SubTask>,
    follow_up_date: Option<NaiveDate>,
    last_update: Option<NaiveDate>,
    // update_freq: Option<Frequency>, // Daily - Weekly - Custom
    completed: bool,
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
