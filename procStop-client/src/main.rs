use chrono::prelude::*;
use std::thread::sleep;
use std::time::Duration;

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
        if all_tasks_done(&tasks) {
            components.leds.finished.turn_on()?;
        } else {
            components.leds.finished.turn_off()?;
        }

        if components.switches.standby.is_on()? {
            standby_loop(components)?;
        } else if tasks.is_empty() {
            components.lcd1602.message_line1("No Tasks today :)")?;
        } else if components.switches.active.is_on()? {
            active_loop(&mut components, &tasks[current_task_i], &db)?;
        } else if components.buttons.next.released()? {
            current_task_i = (current_task_i + 1) % tasks.len();
            update_displays(components, &tasks, current_task_i)?;
        } else if components.buttons.previous.released()? {
            current_task_i = (current_task_i - 1) % tasks.len();
            update_displays(components, &tasks, current_task_i)?;
        }
        sleep(Duration::from_millis(100));
    }
}

fn active_loop(
    components: &mut Components,
    current_task: &Task,
    db: &Database,
) -> Result<(), gpio_cdev::errors::Error> {
    components.leds.status.turn_on()?;
    while components.switches.active.is_on()? {
        if components.buttons.finished.released()? {
            db.task_set_finished(current_task.id).unwrap();
        }
        // TODO Zeit hochzählen und db updaten

        sleep(Duration::from_millis(100));
    }
    components.leds.status.turn_off()?;
    Ok(())
}

fn standby_loop(components: &Components) -> Result<(), gpio_cdev::errors::Error> {
    while components.switches.standby.is_on()? {
        sleep(Duration::from_millis(500));
    }
    Ok(())
}

fn update_displays(
    components: &Components,
    tasks: &Vec<Task>,
    current_task_i: usize,
) -> Result<(), gpio_cdev::errors::Error> {
    if tasks.is_empty() {
        components.lcd1602.message_line1("No Tasks today :)")?;
        components.tm1637.display_time(0, 0);
        components.progress_bar.set_percentage(100)?;
        return Ok(());
    }

    // TODO display proper stuff according to tasks
    components.lcd1602.message("")?;
    components.tm1637.display_time(0, 0);
    components.progress_bar.set_percentage(0)?;
    Ok(())
}

fn all_tasks_done(tasks: &Vec<Task>) -> bool {
    for task in tasks {
        if !task.finished {
            return false;
        }
    }
    true
}
