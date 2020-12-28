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

enum State {
    Start,
    Standby,
    Active,
    Pause,
}

impl State {
    pub fn handle(
        &self,
        conf: &conf::Config,
        components: &mut Components,
        db: &Database,
        tasks: &Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        match *self {
            Self::Start => Self::handle_start(conf, components, db, &tasks, current_task_i),
            Self::Standby => Self::handle_standby(components),
            Self::Active => Self::handle_active(conf, components, db, &tasks, current_task_i),
            Self::Pause => Self::handle_pause(conf, components, &tasks, current_task_i),
        }
    }
    fn handle_start(
        conf: &conf::Config,
        components: &mut Components,
        db: &Database,
        tasks: &Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        // Do startup stuff
        update_displays(components, tasks, *current_task_i).expect("Error updating displays.");
        Self::Standby
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
        conf: &conf::Config,
        components: &mut Components,
        db: &Database,
        tasks: &Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        components
            .leds
            .status
            .turn_on()
            .expect("Error setting status LED.");
        while components
            .switches
            .active
            .is_on()
            .expect("Error reading from active switch")
        {
            if components
                .buttons
                .finished
                .released()
                .expect("Error reading finished button.")
            {
                db.task_set_finished(tasks[*current_task_i].id)
                    .expect("Error setting task as finished in database.");
            }

            // TODO Zeit hochzählen, db updaten
            sleep(Duration::from_millis(100));
        }
        Self::Pause
    }

    fn handle_pause(
        conf: &conf::Config,
        components: &mut Components,
        tasks: &Vec<Task>,
        current_task_i: &mut usize,
    ) -> State {
        components
            .leds
            .status
            .turn_off()
            .expect("Error setting status LED.");
        while !components
            .switches
            .active
            .is_on()
            .expect("Error reading active switch.")
        {
            if components
                .buttons
                .next
                .released()
                .expect("Error reading next button.")
            {
                *current_task_i = (*current_task_i + 1) % tasks.len();
                update_displays(components, tasks, *current_task_i)
                    .expect("Error updating displays.");
            } else if components
                .buttons
                .previous
                .released()
                .expect("Error reading next button.")
            {
                *current_task_i = (*current_task_i - 1) % tasks.len();
                update_displays(components, tasks, *current_task_i)
                    .expect("Error updating displays.");
            }

            sleep(Duration::from_millis(100));
        }
        Self::Active
    }
}

fn main_loop(
    conf: &conf::Config,
    components: &mut Components,
    db: &Database,
) -> Result<(), gpio_cdev::errors::Error> {
    let mut current_task_i = 0;
    let mut state = State::Start;
    let mut current_date;
    let mut tasks;
    loop {
        current_date = format!("{}", Local::today().format("%Y-%m-%d"));
        tasks = db.get_tasks_for_date(&current_date).unwrap();
        state = state.handle(conf, components, db, &tasks, &mut current_task_i);
    }
}
