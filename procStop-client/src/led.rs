use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

pub struct LED {
    output_handle: LineHandle,
    pub name: String,
}

impl LED {
    pub fn new(output_pin: u32, name: &str, path: &str) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new(path)?;
        let output_handle =
            chip.get_line(output_pin)?
                .request(LineRequestFlags::OUTPUT, 0, name)?;
        Ok(LED {
            output_handle,
            name: String::from(name),
        })
    }
    pub fn turn_on(&self) -> Result<(), gpio_cdev::Error> {
        self.output_handle.set_value(1)?;
        Ok(())
    }

    pub fn turn_off(&self) -> Result<(), gpio_cdev::Error> {
        self.output_handle.set_value(0)?;
        Ok(())
    }
}
