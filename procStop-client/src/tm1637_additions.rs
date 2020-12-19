use tm1637_gpio_driver::mappings::SegmentBits;
use tm1637_gpio_driver::TM1637Adapter;

pub trait TM1637AdapterCustomFunctions {
    fn display_time(&self, minutes: u32, hours: u32);
}

impl TM1637AdapterCustomFunctions for TM1637Adapter {
    fn display_time(&self, minutes: u32, hours: u32) {
        let time_string = format!("{:02}{:02}", minutes, hours);
        let mut data = Self::encode_string(&time_string);
        data[1] = data[1] | SegmentBits::SegPoint as u8;
        self.write_segments_raw(&data, 0);
    }
}
