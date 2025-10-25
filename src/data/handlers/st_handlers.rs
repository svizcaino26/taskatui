use crate::{SqlitePool, SubTask};

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

    pub async fn edit_description(
        &self,
        new_description: &str,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"
        UPDATE sub_tasks
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
        sqlx::query!(
            r#"
                UPDATE sub_tasks
                SET completed = true
                WHERE id = ?1
            "#,
            self.id
        )
        .execute(pool)
        .await?;
        Ok(())
    }
}
