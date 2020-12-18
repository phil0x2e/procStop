use gpio_cdev::{Chip, LineRequestFlags};
use std::thread::sleep;
use std::time::Duration;
fn main() {
    let mut chip = Chip::new("/dev/gpiochip0").unwrap();
    let output = chip.get_line(4).unwrap();
    let output_handle = output.request(LineRequestFlags::OUTPUT, 0, "LED").unwrap();
    loop {
        output_handle.set_value(1).unwrap();
        println!("ON");
        sleep(Duration::from_millis(300));
        output_handle.set_value(0).unwrap();
        println!("OFF");
        sleep(Duration::from_millis(300));
    }
}
