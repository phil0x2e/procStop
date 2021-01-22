use super::conf::*;
use super::inputs;
use super::lcd_1602;
use super::led;
use super::progress_bar;
use super::tm1637_additions;
use tm1637_gpio_driver::TM1637Adapter;

pub struct Components {
    pub leds: LEDs,
    pub switches: Switches,
    pub buttons: Buttons,
    pub lcd1602: lcd_1602::LCD,
    pub tm1637: TM1637Adapter,
    pub progress_bar: progress_bar::ProgressBar,
}

pub struct LEDs {
    pub status: led::LED,
    pub finished: led::LED,
}

pub struct Switches {
    pub active: inputs::Switch,
    pub standby: inputs::Switch,
}

pub struct Buttons {
    pub previous: inputs::Button,
    pub next: inputs::Button,
    pub finished: inputs::Button,
}

pub fn init_components(conf: &Config) -> Result<Components, gpio_cdev::Error> {
    let comps = Components {
        leds: LEDs {
            status: led::LED::new(conf.gpio.leds.status, "status_led", &conf.gpio.path)?,
            finished: led::LED::new(conf.gpio.leds.finished, "finished_led", &conf.gpio.path)?,
        },
        switches: Switches {
            active: inputs::Switch::new(
                conf.gpio.switches.active,
                "active_switch",
                &conf.gpio.path,
            )?,
            standby: inputs::Switch::new(
                conf.gpio.switches.standby,
                "standby_switch",
                &conf.gpio.path,
            )?,
        },
        buttons: Buttons {
            previous: inputs::Button::new(
                conf.gpio.buttons.previous,
                "previous_button",
                &conf.gpio.path,
            )?,
            next: inputs::Button::new(conf.gpio.buttons.next, "next_button", &conf.gpio.path)?,
            finished: inputs::Button::new(
                conf.gpio.buttons.finished,
                "finished_button",
                &conf.gpio.path,
            )?,
        },
        lcd1602: lcd_1602::LCD::new(
            conf.gpio.lcd1602.rs,
            conf.gpio.lcd1602.e,
            conf.gpio.lcd1602.d4,
            conf.gpio.lcd1602.d5,
            conf.gpio.lcd1602.d6,
            conf.gpio.lcd1602.d7,
            16,
            &conf.gpio.path,
        )?,
        tm1637: tm1637_additions::setup_gpio_cdev_predefined_delay(
            conf.gpio.tm1637.clk,
            conf.gpio.tm1637.dio,
            &conf.gpio.path,
        ),
        progress_bar: progress_bar::ProgressBar::new(
            conf.gpio.progress_bar.s0,
            conf.gpio.progress_bar.s1,
            conf.gpio.progress_bar.s2,
            conf.gpio.progress_bar.s3,
            &conf.gpio.path,
        )?,
    };
    comps.lcd1602.init_display()?;
    Ok(comps)
}
