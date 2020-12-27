use serde::Deserialize;
use std::fs;
use std::io;
use toml;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub database: DatabaseConf,
    pub gpio: GPIO,
}

#[derive(Deserialize, Debug)]
pub struct DatabaseConf {
    pub path: String,
}

#[derive(Deserialize, Debug)]
pub struct GPIO {
    pub path: String,
    pub leds: LEDGPIO,
    pub switches: SwitchGPIO,
    pub buttons: ButtonGPIO,
    pub lcd1602: LCD1602GPIO,
    pub tm1637: TM1637GPIO,
    pub progress_bar: ProgressBarGPIO,
}

#[derive(Deserialize, Debug)]
pub struct LEDGPIO {
    pub status: u32,
    pub finished: u32,
}

#[derive(Deserialize, Debug)]
pub struct SwitchGPIO {
    pub active: u32,
    pub standby: u32,
}

#[derive(Deserialize, Debug)]
pub struct ButtonGPIO {
    pub previous: u32,
    pub next: u32,
    pub finished: u32,
}

#[derive(Deserialize, Debug)]
pub struct LCD1602GPIO {
    pub e: u32,
    pub rs: u32,
    pub d4: u32,
    pub d5: u32,
    pub d6: u32,
    pub d7: u32,
}

#[derive(Deserialize, Debug)]
pub struct TM1637GPIO {
    pub clk: u32,
    pub dio: u32,
}

#[derive(Deserialize, Debug)]
pub struct ProgressBarGPIO {
    pub s0: u32,
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
}

pub fn get_config(path: &str) -> Result<Config, io::Error> {
    let content: String = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
