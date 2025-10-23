mod data;
use crate::data::models::{NewTask, SubTask, Task, TaskDetailManager};
use sqlx::SqlitePool;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 1️⃣ Create DB pool (adjust path as needed)
    let pool = SqlitePool::connect("sqlite://storage.db").await?;

    // 2️⃣ Create some sample tasks
    let t1 = Task::create_task(&pool, NewTask::new("Test Task A")).await?;
    let t2 = Task::create_task(&pool, NewTask::new("Test Task B")).await?;

    // 3️⃣ Add subtasks
    let _ = t1.add_sub_task(&pool, "Sub A1").await?;
    let sub_a2 = t1.add_sub_task(&pool, "Sub A2").await?;
    let _ = t2.add_sub_task(&pool, "Sub B1").await?;

    // 4️⃣ Pull all fresh tasks + subtasks
    let tasks = Task::get_pending_tasks(&pool).await?;
    let subs = SubTask::get_pending_sub_tasks(&pool).await?;

    let mut manager = TaskDetailManager::build_task_details(tasks, subs);

    println!("===== BEFORE DELETE =====");
    debug_print(&manager);

    // 5️⃣ TEST REMOVE *SUBTASK*
    manager.remove_subtask(t1.id, sub_a2.id, &pool).await?;

    println!("===== AFTER SUBTASK DELETE =====");
    debug_print(&manager);

    // 6️⃣ TEST REMOVE *TASK*
    manager.remove_task(t2.id, &pool).await?;

    println!("===== AFTER TASK DELETE =====");
    debug_print(&manager);

    Ok(())
}

// Just a helper fn to visualize the tree
fn debug_print(manager: &TaskDetailManager) {
    for td in &manager.list {
        println!("Task {}: {}", td.task.id, td.task.title);
        for st in &td.subtasks {
            println!("  - SubTask {}: {}", st.id, st.description);
        }
    }
}
