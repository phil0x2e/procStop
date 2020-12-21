use chrono::prelude::*;
use chrono::TimeZone;
use std::thread::sleep;
use std::time::Duration;

use rusqlite;

use tm1637_gpio_driver::gpio_api::setup_gpio_cdev;
use tm1637_gpio_driver::mappings::SegmentBits;
use tm1637_gpio_driver::TM1637Adapter;

use procStop_client::*;

static LCD_RS: u32 = 21;
static LCD_E: u32 = 20;
static LCD_DATA4: u32 = 12;
static LCD_DATA5: u32 = 7;
static LCD_DATA6: u32 = 8;
static LCD_DATA7: u32 = 25;

static LCD_WIDTH: usize = 16;

static TM_CLK: u32 = 24;
static TM_DIO: u32 = 23;

fn lcd_test() {
    let lcd = LCD::new(
        LCD_RS, LCD_E, LCD_DATA4, LCD_DATA5, LCD_DATA6, LCD_DATA7, LCD_WIDTH,
    )
    .unwrap();
    lcd.init_display().unwrap();
    lcd.message_line1("Klappt der Mist,").unwrap();
    lcd.message_line2("denn jetzt auch??!!").unwrap();

    //lcd.clear().unwrap();
}
fn tm1637_test() {
    let tm1637display = setup_gpio_cdev_predefined_delay(TM_CLK, TM_DIO, "/dev/gpiochip0");
    loop {
        let date = Local::now();
        tm1637display.display_time(date.hour(), date.minute());
        sleep(Duration::from_secs(1));
    }
}

fn db_test() -> rusqlite::Result<()> {
    let db = Database::new("/var/www/html/procstop/database.sqlite3")?;
    let tasks = db.get_tasks_for_date("2020-12-21")?;
    for task in tasks {
        println!("{:?}", task);
    }
    Ok(())
}

fn main() {
    db_test().unwrap();
}
