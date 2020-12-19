use gpio_cdev::{Chip, LineHandle, LineRequestFlags};
use std::thread::sleep;
use std::time::Duration;

pub struct LCD {
    chip: Chip,
    rs_handle: LineHandle,
    e_handle: LineHandle,
    d4_handle: LineHandle,
    d5_handle: LineHandle,
    d6_handle: LineHandle,
    d7_handle: LineHandle,

    width: usize,
    line1: u8,
    line2: u8,
    chr: u8,
    cmd: u8,
    e_pulse_nanos: u64,
    e_delay_nanos: u64,
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
        line1: u8,
        line2: u8,
        chr: u8,
        cmd: u8,
        e_pulse_nanos: u64,
        e_delay_nanos: u64,
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
            chip,
            rs_handle,
            e_handle,
            d4_handle,
            d5_handle,
            d6_handle,
            d7_handle,
            width,
            line1,
            line2,
            chr,
            cmd,
            e_pulse_nanos,
            e_delay_nanos,
        })
    }

    pub fn init_display(&self) -> Result<(), gpio_cdev::Error> {
        self.write_byte(0x33, self.cmd)?;
        self.write_byte(0x32, self.cmd)?;
        self.write_byte(0x28, self.cmd)?;
        self.write_byte(0x0C, self.cmd)?;
        self.write_byte(0x06, self.cmd)?;
        self.write_byte(0x01, self.cmd)?;
        Ok(())
    }

    pub fn write_byte(&self, byte: u8, mode: u8) -> Result<(), gpio_cdev::Error> {
        // set pins low
        self.rs_handle.set_value(mode)?;
        self.set_all_data_low()?;
        if byte & 0x10 == 0x10 {
            self.d4_handle.set_value(1)?;
        }
        if byte & 0x20 == 0x20 {
            self.d5_handle.set_value(1)?;
        }
        if byte & 0x40 == 0x40 {
            self.d6_handle.set_value(1)?;
        }
        if byte & 0x80 == 0x80 {
            self.d7_handle.set_value(1)?;
        }
        self.sleep_delay();
        self.e_handle.set_value(1)?;
        self.sleep_pulse();
        self.e_handle.set_value(0)?;
        self.sleep_delay();
        self.set_all_data_low()?;
        if byte & 0x01 == 0x01 {
            self.d4_handle.set_value(1)?;
        }
        if byte & 0x02 == 0x02 {
            self.d5_handle.set_value(1)?;
        }
        if byte & 0x04 == 0x04 {
            self.d6_handle.set_value(1)?;
        }
        if byte & 0x08 == 0x08 {
            self.d7_handle.set_value(1)?;
        }
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
            self.write_byte(padded_message_bytes[i], self.chr)?;
        }
        Ok(())
    }

    pub fn message_line1(&self, message: &str) -> Result<(), gpio_cdev::Error> {
        self.write_byte(self.line1, self.cmd)?;
        self.message(&message)?;
        Ok(())
    }
    pub fn message_line2(&self, message: &str) -> Result<(), gpio_cdev::Error> {
        self.write_byte(self.line2, self.cmd)?;
        self.message(&message)?;
        Ok(())
    }

    pub fn clear(&self) -> Result<(), gpio_cdev::Error> {
        self.write_byte(0x01, self.cmd);
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

    fn set_all_data_low(&self) -> Result<(), gpio_cdev::Error> {
        self.d4_handle.set_value(0)?;
        self.d5_handle.set_value(0)?;
        self.d6_handle.set_value(0)?;
        self.d7_handle.set_value(0)?;
        Ok(())
    }

    fn sleep_delay(&self) {
        sleep(Duration::from_micros(self.e_delay_nanos));
    }

    fn sleep_pulse(&self) {
        sleep(Duration::from_micros(self.e_pulse_nanos));
    }
}
