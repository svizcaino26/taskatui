use crate::data::models::{NewTask, SubTask, Task, TaskDetail};
use chrono::NaiveDate;
use sqlx::SqlitePool;
use std::collections::HashMap;

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

    pub async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
        DELETE FROM tasks
        WHERE id = ?1
        "#,
            self.id
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

    pub async fn add_sub_task(
        &self,
        pool: &SqlitePool,
        description: &str,
    ) -> anyhow::Result<SubTask> {
        let sub_task = sqlx::query_as!(
            SubTask,
            r#"
            INSERT INTO sub_tasks (task_id, description, completed)
            VALUES(?1, ?2, ?3)
            RETURNING id, task_id, description, completed as "completed: bool"
            "#,
            self.id,
            description,
            false
        )
        .fetch_one(pool)
        .await?;

        Ok(sub_task)
    }
}

impl SubTask {
    pub async fn get_pending_sub_tasks(pool: &SqlitePool) -> anyhow::Result<Vec<Self>> {
        let sub_tasks = sqlx::query_as!(
            Self,
            r#"
            SELECT id, task_id, description, completed as "completed: bool"
            FROM sub_tasks
            WHERE completed = false
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(sub_tasks)
    }

    pub async fn delete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
        DELETE FROM sub_tasks
        WHERE id = ?1
        "#,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }
}

impl TaskDetail {
    pub fn build_task_details(tasks: Vec<Task>, sub_tasks: Vec<SubTask>) -> Vec<Self> {
        let mut subtask_map: HashMap<i64, Vec<SubTask>> = HashMap::new();

        sub_tasks.into_iter().for_each(|st| {
            subtask_map.entry(st.task_id).or_default().push(st);
        });

        tasks
            .into_iter()
            .map(|task| {
                let subtasks = subtask_map.remove(&task.id).unwrap_or_default();
                TaskDetail { task, subtasks }
            })
            .collect()
    }
}
