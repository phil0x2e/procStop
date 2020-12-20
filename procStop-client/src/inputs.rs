use gpio_cdev::{Chip, LineHandle, LineRequestFlags};

pub struct Switch {
    input_handle: LineHandle,
    pub name: String,
}

pub struct Button {
    input_handle: LineHandle,
    last_pressed: bool,
    pub name: String,
}

impl Switch {
    pub fn new(input_pin: u32, name: &str) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let input_handle = chip
            .get_line(input_pin)?
            .request(LineRequestFlags::INPUT, 0, name)?;
        Ok(Switch {
            input_handle,
            name: String::from(name),
        })
    }
    pub fn is_on(&self) -> Result<bool, gpio_cdev::Error> {
        if self.input_handle.get_value()? == 1 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }
}

impl Button {
    pub fn new(input_pin: u32, name: &str) -> Result<Self, gpio_cdev::Error> {
        let mut chip = Chip::new("/dev/gpiochip0")?;
        let input_handle = chip
            .get_line(input_pin)?
            .request(LineRequestFlags::INPUT, 0, name)?;
        Ok(Button {
            input_handle,
            last_pressed: false,
            name: String::from(name),
        })
    }

    pub fn is_pressed(&self) -> Result<bool, gpio_cdev::Error> {
        if self.input_handle.get_value()? == 1 {
            return Ok(true);
        } else {
            return Ok(false);
        }
    }

    pub fn released(&mut self) -> Result<bool, gpio_cdev::Error> {
        let current_pressed = self.is_pressed()?;
        if !current_pressed && self.last_pressed {
            self.last_pressed = false;
            return Ok(true);
        }
        self.last_pressed = current_pressed;
        Ok(false)
    }
}
