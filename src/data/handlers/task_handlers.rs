use crate::{NewTask, SqlitePool, SubTask, Task};
use chrono::NaiveDate;

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

    pub async fn edit_title(&self, new_title: &str, pool: &SqlitePool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
        UPDATE tasks
        SET title = ?1
        WHERE id = ?2
        "#,
            new_title,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn edit_description(
        &self,
        new_description: &str,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
        UPDATE tasks
        SET description = ?1
        WHERE id = ?2
        "#,
            new_description,
            self.id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn complete(&self, pool: &SqlitePool) -> anyhow::Result<()> {
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

    pub async fn complete_children(&self, pool: &SqlitePool) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
                UPDATE sub_tasks
                SET completed = true
                WHERE task_id = ?1
            "#,
            self.id
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
