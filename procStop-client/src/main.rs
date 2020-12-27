use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;

use rusqlite;

use procStop_client::*;

fn main() {
    let conf = conf::get_config("config.toml").unwrap();
    let mut components = init_components(&conf).unwrap();
    let db = Database::new(&conf.database.path).unwrap();
    main_loop(&conf, &mut components, &db).unwrap();
}

fn main_loop(
    conf: &conf::Config,
    mut components: &mut Components,
    db: &Database,
) -> Result<(), gpio_cdev::errors::Error> {
    let mut current_task_i = 0;

    let mut current_date = format!("{}", Local::today().format("%Y-%m-%d"));
    let mut tasks = db.get_tasks_for_date(&current_date).unwrap();
    if tasks.is_empty() {
        components.lcd1602.message_line1("No Tasks today :)")?;
    } else {
        components
            .lcd1602
            .message_line1(&tasks[current_task_i].name)?;
    }
    loop {
        current_date = format!("{}", Local::today().format("%Y-%m-%d"));
        tasks = db.get_tasks_for_date(&current_date).unwrap();
        if components.switches.standby.is_on()? {
            standby_loop(components)?;
        } else if tasks.is_empty() {
            components.lcd1602.message_line1("No Tasks today :)")?;
        } else if components.switches.active.is_on()? {
            active_loop(&mut components, &tasks[current_task_i], &db)?;
        } else if components.buttons.next.released()? {
            current_task_i = (current_task_i + 1) % tasks.len();
            components
                .lcd1602
                .message_line1(&tasks[current_task_i].name)?;
        } else if components.buttons.previous.released()? {
            current_task_i = (current_task_i - 1) % tasks.len();
            components
                .lcd1602
                .message_line1(&tasks[current_task_i].name)?;
        }
        sleep(Duration::from_millis(100));
    }
}

fn active_loop(
    components: &mut Components,
    current_task: &Task,
    db: &Database,
) -> Result<(), gpio_cdev::errors::Error> {
    while components.switches.active.is_on()? {
        if components.buttons.finished.released()? {
            db.task_set_finished(current_task.id).unwrap();
        }
        // TODO Zeit hochzählen und db updaten
        sleep(Duration::from_millis(100));
    }
    Ok(())
}

fn standby_loop(components: &Components) -> Result<(), gpio_cdev::errors::Error> {
    while components.switches.standby.is_on()? {
        sleep(Duration::from_millis(500));
    }
    Ok(())
}
