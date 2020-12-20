use super::utils::*;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::thread::sleep;
use std::time::Duration;

static LCD_LINE1: u8 = 0x80;
static LCD_LINE2: u8 = 0xC0;
static LCD_CHR: u8 = 1;
static LCD_CMD: u8 = 0;
static E_PULSE_NANOS: u64 = 500;
static E_DELAY_NANOS: u64 = 500;

pub struct LCD {
    rs_handle: LineHandle,
    e_handle: LineHandle,
    d4_handle: LineHandle,
    d5_handle: LineHandle,
    d6_handle: LineHandle,
    d7_handle: LineHandle,

    width: usize,
}

impl LCD {
    pub fn new(
        rs: u32,
        e: u32,
        d4: u32,
        d5: u32,
        d6: u32,
        d7: u32,
        width: usize,
    ) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let rs_handle = chip
            .get_line(rs)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_RS")?;
        let e_handle = chip
            .get_line(e)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_E")?;
        let d4_handle = chip
            .get_line(d4)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_D4")?;
        let d5_handle = chip
            .get_line(d5)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_D5")?;
        let d6_handle = chip
            .get_line(d6)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_D6")?;
        let d7_handle = chip
            .get_line(d7)?
            .request(LineRequestFlags::OUTPUT, 0, "LCD_D7")?;

        Ok(LCD {
            rs_handle,
            e_handle,
            d4_handle,
            d5_handle,
            d6_handle,
            d7_handle,
            width,
        })
    }

    pub fn init_display(&self) -> Result<(), gpio_cdev::Error> {
        self.write_byte(0x33, LCD_CMD)?;
        self.write_byte(0x32, LCD_CMD)?;
        self.write_byte(0x28, LCD_CMD)?;
        self.write_byte(0x0C, LCD_CMD)?;
        self.write_byte(0x06, LCD_CMD)?;
        self.write_byte(0x01, LCD_CMD)?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8, mode: u8) -> Result<(), gpio_cdev::Error> {
        let bits = byte_to_bits(byte);
        self.rs_handle.set_value(mode)?;
        self.d4_handle.set_value(bits[3])?;
        self.d5_handle.set_value(bits[2])?;
        self.d6_handle.set_value(bits[1])?;
        self.d7_handle.set_value(bits[0])?;
        self.sleep_delay();
        self.e_handle.set_value(1)?;
        self.sleep_pulse();
        self.e_handle.set_value(0)?;
        self.sleep_delay();
        self.d4_handle.set_value(bits[7])?;
        self.d5_handle.set_value(bits[6])?;
        self.d6_handle.set_value(bits[5])?;
        self.d7_handle.set_value(bits[4])?;
        self.sleep_delay();
        self.e_handle.set_value(1)?;
        self.sleep_pulse();
        self.e_handle.set_value(0)?;
        self.sleep_delay();
        Ok(())
    }

    pub fn message(&self, message: &str) -> Result<(), gpio_cdev::Error> {
        let padded_message = format!("{:0width$}", message, width = self.width);
        let padded_message_bytes = padded_message.as_bytes();
        for i in 0..self.width {
            self.write_byte(padded_message_bytes[i], LCD_CHR)?;
        }
        Ok(())
    }

    pub fn message_line1(&self, message: &str) -> Result<(), gpio_cdev::Error> {
        self.write_byte(LCD_LINE1, LCD_CMD)?;
        self.message(&message)?;
        Ok(())
    }
    pub fn message_line2(&self, message: &str) -> Result<(), gpio_cdev::Error> {
        self.write_byte(LCD_LINE2, LCD_CMD)?;
        self.message(&message)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), gpio_cdev::Error> {
        self.write_byte(0x01, LCD_CMD)?;
        Ok(())
    }

    pub fn set_all_handles_low(&self) -> Result<(), gpio_cdev::Error> {
        self.e_handle.set_value(0)?;
        self.rs_handle.set_value(0)?;
        self.d4_handle.set_value(0)?;
        self.d5_handle.set_value(0)?;
        self.d6_handle.set_value(0)?;
        self.d7_handle.set_value(0)?;
        Ok(())
    }

    fn sleep_delay(&self) {
        sleep(Duration::from_micros(E_DELAY_NANOS));
    }

    fn sleep_pulse(&self) {
        sleep(Duration::from_micros(E_PULSE_NANOS));
    }
}
