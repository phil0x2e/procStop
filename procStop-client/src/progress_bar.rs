use super::utils::*;
use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

pub struct ProgressBar {
    s0_handle: LineHandle,
    s1_handle: LineHandle,
    s2_handle: LineHandle,
    s3_handle: LineHandle,
}

impl ProgressBar {
    pub fn new(
        s0_pin: u32,
        s1_pin: u32,
        s2_pin: u32,
        s3_pin: u32,
        path: &str,
    ) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new(path)?;
        let s0_handle =
            chip.get_line(s0_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "ProgressBar S0")?;
        let s1_handle =
            chip.get_line(s1_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "ProgressBar S1")?;
        let s2_handle =
            chip.get_line(s2_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "ProgressBar S2")?;
        let s3_handle =
            chip.get_line(s3_pin)?
                .request(LineRequestFlags::OUTPUT, 0, "ProgressBar S3")?;
        Ok(ProgressBar {
            s0_handle,
            s1_handle,
            s2_handle,
            s3_handle,
        })
    }

    pub fn set(&self, num: u8) -> Result<(), gpio_cdev::Error> {
        let bits = byte_to_bits(num);
        self.s0_handle.set_value(bits[7])?;
        self.s1_handle.set_value(bits[6])?;
        self.s2_handle.set_value(bits[5])?;
        self.s3_handle.set_value(bits[4])?;
        Ok(())
    }
    pub fn set_percentage(&self, percentage: u8) -> Result<(), gpio_cdev::Error> {
        self.set(((percentage as u32 * 15) / 100) as u8)?;
        Ok(())
    }
}
