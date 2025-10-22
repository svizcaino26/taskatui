open storage.db | get tasks | each { |task|
    let subs = open storage.db | get sub_tasks | where task_id == $task.id
    $task | merge { sub_tasks: $subs }
}
