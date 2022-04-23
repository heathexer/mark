use crate::config::WeatherOptions;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{
    mpsc::{Receiver, SyncSender},
    Arc,
};
use std::thread;
use std::time::Duration;

use embedded_graphics::{
    image::{Image, ImageRaw},
    pixelcolor::Rgb888,
    prelude::*,
};
use image::io::Reader as ImageReader;
use image::DynamicImage;
use itertools::Itertools;
use rpi_led_matrix::{LedCanvas, LedColor, LedFont};
use serde::Deserialize;
use ureq;

pub struct WeatherWidget {
    position: (i32, i32),
    size: (i32, i32),
    font: LedFont,
    lat: Arc<String>,
    lon: Arc<String>,
    api_key: Arc<String>,
    icons: HashMap<String, Vec<u8>>,
    default_icon: Vec<u8>,
    weather_info: Option<WeatherInfo>,
    cold_color: LedColor,
    warm_color: LedColor,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WeatherInfo {
    weather: Vec<WeatherInfoDesc>,
    main: WeatherInfoTemp,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WeatherInfoDesc {
    main: String,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct WeatherInfoTemp {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
}

impl WeatherWidget {
    pub fn new(position: (i32, i32), size: (i32, i32), config: &WeatherOptions) -> Self {
        WeatherWidget {
            position: position,
            size: size,
            font: LedFont::new(Path::new(&config.font_path)).expect("Failed to load font"),
            lat: Arc::new(config.lat.clone()),
            lon: Arc::new(config.lon.clone()),
            api_key: Arc::new(config.api_key.clone()),
            icons: config
                .icons
                .iter()
                .map(|(k, v)| {
                    (k.clone(), {
                        let img = ImageReader::open(v).unwrap().decode().unwrap();
                        img.into_rgb8().into_raw()
                    })
                })
                .collect(),
            default_icon: {
                let img = ImageReader::open(&config.default_icon)
                    .unwrap()
                    .decode()
                    .unwrap();
                img.into_rgb8().into_raw()
            },
            weather_info: None,
            cold_color: config.cold_color.into(),
            warm_color: config.warm_color.into(),
        }
    }

    pub fn render(&mut self, canvas: &mut LedCanvas, chan: &Receiver<WeatherInfo>) {
        if let Ok(weather_info) = chan.try_recv() {
            self.weather_info = Some(weather_info);
        }

        if let Some(weather_info) = &self.weather_info {
            let (px, py) = self.position;
            let (sx, sy) = self.size;

            // Temp °
            let temp = weather_info.main.temp;
            let text = format!("{:.0}°F", &temp);
            let (temp_min, temp_max) = (weather_info.main.temp_min, weather_info.main.temp_max);
            // let (temp_min, temp_max) = (35.0, 85.0);
            let text_min = format!("{:.0}", &temp_min);
            let text_max = format!("{:.0}", &temp_max);
            let (scale_min, scale_max) = (40, 80);
            canvas.draw_text(
                &self.font,
                &text,
                px + sx - 5 * (text.len() as i32 - 1) + 2,
                py + 8,
                // Scale each RBG value from cold color to warm color propotional to where the current temp lies between 32 and 100 F
                &Self::scale_color(
                    temp,
                    scale_min,
                    scale_max,
                    &self.cold_color,
                    &self.warm_color,
                ),
                0,
                false,
            );

            canvas.draw_text(
                &self.font,
                &text_min,
                px + 5,
                py + 16,
                &Self::scale_color(
                    temp_min,
                    scale_min,
                    scale_max,
                    &self.cold_color,
                    &self.warm_color,
                ),
                0,
                false,
            );

            canvas.draw_text(
                &self.font,
                &text_max,
                px + 20,
                py + 16,
                &Self::scale_color(
                    temp_max,
                    scale_min,
                    scale_max,
                    &self.cold_color,
                    &self.warm_color,
                ),
                0,
                false,
            );

            // Icon

            let img = self
                .icons
                .get(&weather_info.weather[0].main)
                .unwrap_or(&self.default_icon);

            for (i, (r, g, b)) in img.iter().tuples().enumerate() {
                if *r > 0 || *g > 0 || *b > 0 {
                    canvas.set(
                        px + (i as i32 % 8),
                        py + (i as i32 / 8),
                        &LedColor {
                            red: *r,
                            green: *g,
                            blue: *b,
                        },
                    );
                }
            }
        }
    }

    pub fn start_thread(&self, chan: SyncSender<WeatherInfo>) {
        thread::spawn({
            let lat = Arc::clone(&self.lat);
            let lon = Arc::clone(&self.lon);
            let api_key = Arc::clone(&self.api_key);

            move || loop {
                let weather = Self::get_weather(&lat, &lon, &api_key);
                dbg!(&weather);
                chan.send(weather);
                thread::sleep(Duration::new(600, 0));
            }
        });
    }

    fn get_weather(lat: &str, lon: &str, api_key: &str) -> WeatherInfo {
        let resp = ureq::get(&format!("https://api.openweathermap.org/data/2.5/weather?lat={lat}&lon={lon}&appid={api_key}&units=imperial")).call().unwrap();

        resp.into_json().unwrap()
    }

    fn scale_to_range(n: f32, in0: u8, in1: u8, out0: u8, out1: u8) -> u8 {
        let scale = (n - in0 as f32) / (in1 - in0) as f32;
        (scale * (out1 as f32 - out0 as f32) + out0 as f32) as u8
    }

    fn scale_color(n: f32, in0: u8, in1: u8, c0: &LedColor, c1: &LedColor) -> LedColor {
        LedColor {
            red: Self::scale_to_range(n, in0, in1, c0.red, c1.red),
            green: Self::scale_to_range(n, in0, in1, c0.green, c1.green),
            blue: Self::scale_to_range(n, in0, in1, c0.blue, c1.blue),
        }
    }
}
