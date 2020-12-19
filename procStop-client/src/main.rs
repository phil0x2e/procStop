use chrono::prelude::*;
use chrono::TimeZone;
use std::thread::sleep;
use std::time::Duration;

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
static LCD_LINE1: u8 = 0x80;
static LCD_LINE2: u8 = 0xC0;
static LCD_CHR: u8 = 1;
static LCD_CMD: u8 = 0;
static E_PULSE_NANOS: u64 = 500;
static E_DELAY_NANOS: u64 = 500;

fn lcd_test() {
    let lcd = LCD::new(
        LCD_RS,
        LCD_E,
        LCD_DATA4,
        LCD_DATA5,
        LCD_DATA6,
        LCD_DATA7,
        LCD_WIDTH,
        LCD_LINE1,
        LCD_LINE2,
        LCD_CHR,
        LCD_CMD,
        E_PULSE_NANOS,
        E_DELAY_NANOS,
    )
    .unwrap();
    lcd.init_display().unwrap();
    lcd.message_line1("> Hallo,").unwrap();
    lcd.message_line2("Welt! <").unwrap();
    sleep(Duration::from_secs(3));

    //lcd.clear().unwrap();
}

fn main() {
    let (clk_pin, dio_pin) = (24, 23);
    let bit_delay_fn = Box::from(|| sleep(Duration::from_micros(10)));
    let tm1637display = setup_gpio_cdev(clk_pin, dio_pin, bit_delay_fn, "/dev/gpiochip0");
    loop {
        let date = Local::now();
        tm1637display.display_time(date.hour(), date.minute());
        sleep(Duration::from_secs(1));
    }
}
