use chrono::prelude::*;
use std::thread::sleep;
use std::time::{Duration, Instant};

use procstop_client::*;

#[derive(Debug)]
enum State {
    Start,
    Standby,
    Active,
    Pause,
    Unplugged,
}

impl State {
    pub fn handle(
        &self,
        components: &mut Components,
        db: &Database,
        tasks: &mut Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        match *self {
            Self::Start => Self::handle_start(components, tasks, *current_task_i),
            Self::Standby => Self::handle_standby(components),
            Self::Active => Self::handle_active(components, db, tasks, *current_task_i),
            Self::Pause => Self::handle_pause(components, tasks, current_task_i),
            Self::Unplugged => Self::handle_unplugged(components),
        }
    }

    fn handle_start(
        components: &mut Components,
        tasks: &Vec<Task>,
        current_task_i: usize,
    ) -> State {
        // Do startup stuff
        update_displays(components, tasks, current_task_i).expect("Error updating displays.");
        Self::Pause
    }

    fn handle_unplugged(components: &mut Components) -> State {
        // It's connected, when hotplug is 0 and not connected when 1
        // (Builtin pull up resistor on pin 2)
        while components
            .switches
            .hotplug
            .is_on()
            .expect("Error reading hotplug pin.")
        {
            sleep(Duration::from_secs(2));
        }
        // Wait, so the plug is fully connected when initializing display
        sleep(Duration::from_secs(1));
        components
            .lcd1602
            .init_display()
            .expect("Error initializing lcd display.");
        Self::Pause
    }

    fn handle_standby(components: &mut Components) -> State {
        // TODO turn stuff off
        while components
            .switches
            .standby
            .is_on()
            .expect("Error reading from standby switch.")
        {
            sleep(Duration::from_millis(500));
        }
        // TODO turn stuff back on
        Self::Pause
    }

    fn handle_active(
        components: &mut Components,
        db: &Database,
        tasks: &mut Vec<Task>,
        current_task_i: usize,
    ) -> State {
        components
            .leds
            .status
            .turn_on()
            .expect("Error setting status LED.");
        let start = Instant::now();
        let mut minute_count = 1;
        while components
            .switches
            .active
            .is_on()
            .expect("Error reading from active switch")
        {
            if components
                .switches
                .hotplug
                .is_on()
                .expect("Error reading hotplug pin.")
            {
                return State::Unplugged;
            }
            if components
                .buttons
                .finished
                .released()
                .expect("Error reading finished button.")
            {
                db.task_set_finished(tasks[current_task_i].id)
                    .expect("Error setting task as finished in database.");
                update_displays(components, tasks, current_task_i)
                    .expect("Error updating displays.");
                return Self::Active;
            }

            // update time_spent in db every minute, update displays
            // and set finished in db if enough time was spent
            if start.elapsed().as_secs() >= 60 * minute_count {
                db.task_increase_time_spent(tasks[current_task_i].id, 1)
                    .expect("Error writing to database.");
                let current_date = format!("{}", Local::today().format("%Y-%m-%d"));
                *tasks = db.get_tasks_for_date(&current_date).unwrap();
                if tasks[current_task_i].time_spent >= tasks[current_task_i].minimum_time {
                    db.task_set_finished(tasks[current_task_i].id)
                        .expect("Error writing to database.");
                }
                update_displays(components, tasks, current_task_i)
                    .expect("Error updating displays.");
                minute_count += 1;
            }
            sleep(Duration::from_millis(100));
        }
        Self::Pause
    }

    fn handle_pause(
        components: &mut Components,
        tasks: &Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        components
            .leds
            .status
            .turn_off()
            .expect("Error setting status LED.");
        let mut finished = false;
        while !components
            .switches
            .active
            .is_on()
            .expect("Error reading active switch.")
        {
            if components
                .switches
                .hotplug
                .is_on()
                .expect("Error reading hotplug pin.")
            {
                return State::Unplugged;
            }
            if components
                .buttons
                .next
                .released()
                .expect("Error reading next button.")
            {
                if tasks.len() > 0 {
                    *current_task_i = (*current_task_i + 1) % tasks.len();
                    update_displays(components, tasks, *current_task_i)
                        .expect("Error updating displays.");
                }
                return Self::Pause; // Refresh by going back to main loop
            } else if components
                .buttons
                .previous
                .released()
                .expect("Error reading next button.")
            {
                if tasks.len() > 0 {
                    *current_task_i =
                        modulo(*current_task_i as i64 - 1, tasks.len() as i64) as usize;
                    update_displays(components, tasks, *current_task_i)
                        .expect("Error updating displays.");
                }
                return Self::Pause; // Refresh by going back to main loop
            }

            if !finished && all_tasks_done(tasks) {
                execute_finished_animation(components);
                finished = true;
            }
            sleep(Duration::from_millis(100));
        }
        Self::Active
    }
}

fn main_loop(components: &mut Components, db: &Database) -> Result<(), gpio_cdev::errors::Error> {
    let mut current_task_i = 0;
    let mut state = State::Start;
    let mut current_date;
    let mut tasks;
    loop {
        current_date = format!("{}", Local::today().format("%Y-%m-%d"));
        tasks = db.get_tasks_for_date(&current_date).unwrap();
        if !all_tasks_done(&tasks) {
            components
                .leds
                .finished
                .turn_off()
                .expect("Error turning off LED");
        }
        update_displays(components, &tasks, current_task_i).expect("Error updating displays.");
        state = state.handle(components, db, &mut tasks, &mut current_task_i);
    }
}

fn main() {
    let args = get_commandline_args();
    let conf = conf::get_config(&args.config_path).expect("Error! Config File not found.");
    let mut components = init_components(&conf).unwrap();
    let db = Database::new(&conf.database.path).unwrap();
    main_loop(&mut components, &db).unwrap();
}
