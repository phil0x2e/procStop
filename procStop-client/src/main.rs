use std::thread::sleep;
use std::time::Duration;

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

fn main() {
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
