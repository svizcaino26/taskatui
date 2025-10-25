use crate::data::models::{NewTask, SubTask, Task, TaskDetail, TaskDetailManager};
use sqlx::SqlitePool;
use std::collections::HashMap;

impl TaskDetailManager {
    pub async fn init(pool: &SqlitePool) -> anyhow::Result<Self> {
        let tasks = Task::get_pending_tasks(pool).await?;
        let sub_tasks = SubTask::get_pending_sub_tasks(pool).await?;

        let mut subtask_map: HashMap<i64, Vec<SubTask>> = HashMap::new();

        sub_tasks.into_iter().for_each(|st| {
            subtask_map.entry(st.task_id).or_default().push(st);
        });

        Ok(Self {
            list: tasks
                .into_iter()
                .map(|task| {
                    let subtasks = subtask_map.remove(&task.id).unwrap_or_default();
                    TaskDetail { task, subtasks }
                })
                .collect(),
        })
    }

    pub async fn remove_subtask(
        &mut self,
        task_id: i64,
        subtask_id: i64,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter_mut().find(|td| td.task.id == task_id) {
            if let Some(subtask) = task_detail.subtasks.iter().find(|st| st.id == subtask_id) {
                subtask.delete(pool).await?
            }

            task_detail.subtasks.retain(|st| st.id != subtask_id)
        }
        Ok(())
    }

    pub async fn remove_task(&mut self, task_id: i64, pool: &SqlitePool) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter().find(|td| td.task.id == task_id) {
            task_detail.task.delete(pool).await?;
        }
        self.list.retain(|td| td.task.id != task_id);
        Ok(())
    }

    pub async fn edit_subtask_description(
        &mut self,
        task_id: i64,
        subtask_id: i64,
        new_descripton: &str,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(subtask) = self
            .list
            .iter_mut()
            .find(|td| td.task.id == task_id)
            .and_then(|td| td.subtasks.iter_mut().find(|st| st.id == subtask_id))
        {
            subtask.edit_description(new_descripton, pool).await?;
            subtask.description = new_descripton.to_string()
        }
        Ok(())
    }

    pub async fn edit_task_title(
        &mut self,
        new_title: &str,
        task_id: i64,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter_mut().find(|td| td.task.id == task_id) {
            task_detail.task.edit_title(new_title, pool).await?;
            task_detail.task.title = new_title.to_string()
        }

        Ok(())
    }

    pub async fn edit_task_description(
        &mut self,
        new_descripton: &str,
        task_id: i64,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter_mut().find(|td| td.task.id == task_id) {
            task_detail
                .task
                .edit_description(new_descripton, pool)
                .await?;
            task_detail.task.description = Some(new_descripton.to_string())
        }

        Ok(())
    }

    pub async fn add_task(&mut self, title: &str, pool: &SqlitePool) -> anyhow::Result<()> {
        let new_task = Task::create_task(pool, NewTask::new(title)).await?;
        self.list.push(TaskDetail {
            task: new_task,
            subtasks: Vec::new(),
        });

        Ok(())
    }

    pub async fn add_subtask(
        &mut self,
        task_id: i64,
        description: &str,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter_mut().find(|td| td.task.id == task_id) {
            let new_subtask = task_detail.task.add_sub_task(pool, description).await?;
            task_detail.subtasks.push(new_subtask);
        }
        Ok(())
    }

    pub async fn complete_subtask(
        &mut self,
        task_id: i64,
        subtask_id: i64,
        pool: &SqlitePool,
    ) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter_mut().find(|td| td.task.id == task_id) {
            if let Some(st) = task_detail.subtasks.iter().find(|st| st.id == subtask_id) {
                st.complete(pool).await?;
            }
            task_detail.subtasks.retain(|st| st.id != subtask_id);
        }
        Ok(())
    }

    pub async fn complete_task(&mut self, task_id: i64, pool: &SqlitePool) -> anyhow::Result<()> {
        if let Some(task_detail) = self.list.iter().find(|td| td.task.id == task_id) {
            task_detail.task.complete(pool).await?;
            task_detail.task.complete_children(pool).await?;
        }
        self.list.retain(|td| td.task.id != task_id);

        Ok(())
    }
}
