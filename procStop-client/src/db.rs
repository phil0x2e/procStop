use rusqlite::{params, Connection, Result};

pub struct Database {
    connection: Connection,
}

#[derive(Debug)]
pub struct Task {
    id: i32,
    name: String,
    date: String,
    minimum_time: i32,
    time_spent: i32,
    finished: bool,
}

fn i32_to_bool(n: i32) -> bool {
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
                finished: i32_to_bool(row.get(5)?),
            })
        })?;
        let unwrapped_tasks = tasks.filter_map(|t| t.ok()).collect();
        Ok((unwrapped_tasks))
    }

    pub fn task_set_finished(&self, task_id: i32) -> Result<()> {
        // Stub TODO
        Ok(())
    }

    pub fn task_set_time_spent(&self, task_id: i32, time_spent: i32) -> Result<()> {
        // Stub TODO
        Ok(())
    }
}
