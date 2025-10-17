use chrono::NaiveDate;
use sqlx::FromRow;

#[derive(Debug, FromRow)]
pub struct Task {
    id: u32,
    title: String,
    description: Option<String>,
    subtasks: Option<Vec<String>>,
    follow_up_date: NaiveDate,
    last_update: Option<NaiveDate>,
    update_freq: Frequency, // Daily - Weekly - Custom
    completed: bool,
}
