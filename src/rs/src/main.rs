use config::*;
use countdown::CountdownWidget;
use life::LifeWidget;
use presence::PresenceWidget;
use rpi_led_matrix::{LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use std::fs::File;
use std::sync::{mpsc, Arc};
use time::TimeWidget;
use weather::WeatherWidget;

pub mod config;
pub mod countdown;
pub mod life;
pub mod presence;
pub mod server;
pub mod time;
pub mod weather;

fn main() {
    // Options
    let mut options = LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_chain_length(2);
    options.set_parallel(1);
    options.set_multiplexing(0);
    options.set_row_addr_type(0);
    options.set_limit_refresh(100);
    options.set_hardware_mapping("adafruit-hat");
    options.set_pwm_lsb_nanoseconds(500);
    options.set_pixel_mapper_config("V-mapper");
    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let file = File::open("../config.json").expect("Failed to open config file");
    let reader = std::io::BufReader::new(file);
    let config: MarkConfig = serde_json::from_reader(reader).unwrap();

    let mut lw = LifeWidget::new((0, 0), (64, 63), config.life_options);
    let mut tw = TimeWidget::new((1, 1), (62, 7), config.time_options);
    let cw = CountdownWidget::new((1, 8), (62, 7), config.countdown_options);

    let (psender, preceiver) = mpsc::sync_channel(20);
    let mut pw = PresenceWidget::new((1, 15), (30, 51), &config.presence_options);
    pw.start_thread(config.presence_options.user_devices, psender);

    let (wsender, wreceiver) = mpsc::sync_channel(20);
    let mut ww = WeatherWidget::new((32, 15), (30, 16), &config.weather_options);
    ww.start_thread(wsender);

    let (ssender, sreceiver) = mpsc::sync_channel(20);
    let mut flag = true;
    let _rt = crate::server::start_server(ssender);

    loop {
        canvas.clear();

        if flag {
            lw.render(&mut canvas);
            tw.render(&mut canvas);
            cw.render(&mut canvas);
            pw.render(&mut canvas, &preceiver);
            ww.render(&mut canvas, &wreceiver);
        }

        crate::server::update_server(&sreceiver, &mut flag);

        canvas = matrix.swap(canvas);
    }
}
