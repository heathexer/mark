use rpi_led_matrix::LedColor;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Into;
use std::sync::Arc;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarkConfig {
    pub life_options: LifeOptions,
    pub time_options: TimeOptions,
    pub countdown_options: CountdownOptions,
    pub presence_options: PresenceOptions,
    pub weather_options: WeatherOptions,
}

#[derive(Serialize, Deserialize, Copy, Clone)]
pub struct ConfigColor {
    red: u8,
    green: u8,
    blue: u8,
}

impl Into<LedColor> for ConfigColor {
    fn into(self) -> LedColor {
        LedColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}

impl Into<LedColor> for &ConfigColor {
    fn into(self) -> LedColor {
        LedColor {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeOptions {
    pub time_color: ConfigColor,
    pub month_color: ConfigColor,
    pub day_color: ConfigColor,
    pub time_font_path: String,
    pub date_font_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LifeOptions {
    pub alive_color: ConfigColor,
    pub dead_color: ConfigColor,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountdownOptions {
    pub line_color: ConfigColor,
    pub main_color: ConfigColor,
    pub font_path: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresenceOptions {
    pub font_path: String,
    pub username: String,
    pub password: String,
    pub user_devices: HashMap<String, String>,
    pub user_colors: HashMap<String, ConfigColor>,
    pub main_color: ConfigColor,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherOptions {
    pub font_path: String,
    pub lat: String,
    pub lon: String,
    pub api_key: String,
    pub icons: HashMap<String, String>,
    pub default_icon: String,
    pub cold_color: ConfigColor,
    pub warm_color: ConfigColor,
}
