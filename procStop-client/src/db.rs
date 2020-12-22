use rusqlite::{params, Connection, Result};

pub struct Database {
    connection: Connection,
}

#[derive(Debug)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub date: String,
    pub minimum_time: u32,
    pub time_spent: u32,
    pub finished: bool,
}

fn u32_to_bool(n: u32) -> bool {
    if n == 0 {
        false
    } else {
        true
    }
}
impl Database {
    pub fn new(path: &str) -> Result<Self> {
        let connection = Connection::open(path)?;
        Ok(Database { connection })
    }

    pub fn get_tasks_for_date(&self, date: &str) -> Result<Vec<Task>> {
        let mut stmt = self.connection.prepare("SELECT tasks.id, tasknames.name, dates.date, tasks.minimum_time, tasks.time_spent, tasks.finished
        FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id WHERE dates.date=?1;")?;
        let tasks = stmt.query_map(params![date], |row| {
            Ok(Task {
                id: row.get(0)?,
                name: row.get(1)?,
                date: row.get(2)?,
                minimum_time: row.get(3)?,
                time_spent: row.get(4)?,
                finished: u32_to_bool(row.get(5)?),
            })
        })?;
        let unwrapped_tasks = tasks.filter_map(|t| t.ok()).collect();
        Ok(unwrapped_tasks)
    }

    pub fn task_set_finished(&self, task_id: u32) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("UPDATE tasks SET finished=1 WHERE id=?1;")?;
        stmt.execute(params![task_id])?;
        Ok(())
    }

    pub fn task_set_time_spent(&self, task_id: u32, time_spent: u32) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("UPDATE tasks SET Time_spent=?1 WHERE id=?2;")?;
        stmt.execute(params![time_spent, task_id])?;
        Ok(())
    }

    pub fn task_increase_time_spent(&self, task_id: u32, increment: u32) -> Result<()> {
        let mut stmt = self
            .connection
            .prepare("SELECT time_spent FROM tasks WHERE id=?1;")?;
        let old_time_spent: u32 = stmt.query_row(params![task_id], |row| row.get(0))?;

        let mut stmt = self
            .connection
            .prepare("UPDATE tasks SET Time_spent=?1 WHERE id=?2;")?;
        stmt.execute(params![old_time_spent + increment, task_id])?;
        Ok(())
    }
}
