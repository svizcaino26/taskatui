use crate::data::models::{NewTask, Task};
use chrono::NaiveDate;
use sqlx::SqlitePool;

impl Task {
    pub async fn create_task(pool: &SqlitePool, new_task: NewTask) -> anyhow::Result<Self> {
        let title = new_task.title();
        let task = sqlx::query_as!(
            Self,
            r#"
            INSERT INTO tasks(title, completed, last_update)
            VALUES(?1, ?2, DATE('now', 'localtime'))
            RETURNING id, title, description,
            follow_up_date as "follow_up_date: NaiveDate",
            last_update as "last_update: NaiveDate",
            completed as "completed: bool"
            "#,
            title,
            false
        )
        .fetch_one(pool)
        .await?;

        Ok(task)
    }

    pub async fn complete_task(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        let id = self.id;
        sqlx::query!(
            r#"
        UPDATE tasks
        SET completed = true, last_update = DATE('now', 'localtime')
        WHERE id = ?1
        "#,
            id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn get_pending_tasks(pool: &SqlitePool) -> anyhow::Result<Vec<Task>> {
        let tasks = sqlx::query_as!(
            Self,
            r#"
            SELECT id, title, description,
            follow_up_date as "follow_up_date: NaiveDate",
            last_update as "last_update: NaiveDate",
            completed as "completed: bool"
            FROM tasks
            WHERE completed = false
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(tasks)
    }
}
