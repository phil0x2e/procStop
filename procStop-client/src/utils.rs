use super::components::Components;
use super::db::Task;
use super::tm1637_additions::*;
use std::thread::sleep;
use std::time::Duration;

pub fn modulo(lhs: i64, rhs: i64) -> u32 {
    ((lhs % rhs + rhs) % rhs) as u32
}

pub fn byte_to_bits(n: u8) -> [u8; 8] {
    let mut array: [u8; 8] = [0; 8];
    let mut mask = 0x01;
    for i in (0..8).rev() {
        if n & mask != 0 {
            array[i] = 1;
        }
        mask <<= 1;
    }
    array
}

pub fn task_get_time_left(task: &Task) -> u32 {
    if task.finished {
        return 0;
    }
    task.minimum_time - task.time_spent
}

pub fn tasks_get_total_minimum_time(tasks: &Vec<Task>) -> u32 {
    tasks.iter().fold(0, |acc, task| acc + task.minimum_time)
}

pub fn tasks_get_percentage_done(tasks: &Vec<Task>) -> u8 {
    let total_min = tasks_get_total_minimum_time(tasks);
    let mut total_spent = 0;
    for task in tasks {
        if task.finished {
            total_spent += task.minimum_time;
        } else {
            total_spent += task.time_spent;
        }
    }
    (total_spent as f64 / total_min as f64 * 100.) as u8
}

pub fn minutes_to_hours(minutes: u32) -> [u32; 2] {
    [minutes / 60, minutes % 60]
}

pub fn all_tasks_done(tasks: &Vec<Task>) -> bool {
    for task in tasks {
        if !task.finished {
            return false;
        }
    }
    true
}

pub fn update_displays(
    components: &Components,
    tasks: &Vec<Task>,
    current_task_i: usize,
) -> Result<(), gpio_cdev::errors::Error> {
    if tasks.is_empty() {
        components.lcd1602.message_line1("No Tasks today!")?;
        components.lcd1602.message_line2(":)")?;
        components.tm1637.display_time(0, 0);
        components.progress_bar.set_percentage(100)?;
    } else {
        components
            .lcd1602
            .message_line1(&tasks[current_task_i].name)
            .expect("Error writing to lcd screen.");
        components
            .lcd1602
            .message_line2(&format!("{}/{}", current_task_i + 1, tasks.len()))
            .expect("Error writing to lcd screen.");
        let hours = minutes_to_hours(task_get_time_left(&tasks[current_task_i]));
        components.tm1637.display_time(hours[0], hours[1]);
        components
            .progress_bar
            .set_percentage(tasks_get_percentage_done(tasks))
            .expect("Error setting progress bar.");
    }
    Ok(())
}

pub fn execute_finished_animation(components: &Components) {
    for i in 0..16 {
        // small success animation
        components
            .progress_bar
            .set(i)
            .expect("Error setting progress bar.");
        components
            .leds
            .finished
            .turn_off()
            .expect("Error setting finished LED.");
        sleep(Duration::from_millis(100));
        components
            .leds
            .finished
            .turn_on()
            .expect("Error setting finished LED.");
        sleep(Duration::from_millis(100));
    }
}
