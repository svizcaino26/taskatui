let title = $env.TASK_TITLE
sqlite3 storage.db "INSERT INTO tasks(title, completed, last_update) VALUES('"($title)"', 0, DATE('now','localtime'))"
echo $"Task added: ($title)"
