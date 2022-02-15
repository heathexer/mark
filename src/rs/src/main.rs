#[macro_use(azip)]
extern crate ndarray;

use chrono::prelude::*;
use life::LifeWidget;
use rpi_led_matrix::{LedColor, LedFont, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use std::collections::HashMap;
use std::path::Path;
use time::TimeWidget;

pub mod life;
pub mod time;

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

    // Config
    let colors = HashMap::from([
        (
            "line",
            LedColor {
                red: 200,
                green: 200,
                blue: 200,
            },
        ),
        (
            "daysrm",
            LedColor {
                red: 217,
                green: 148,
                blue: 48,
            },
        ),
    ]);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let start_date = NaiveDate::from_ymd(2022, 1, 3);
    let end_date = NaiveDate::from_ymd(2022, 3, 11);

    let mut lw = LifeWidget::new((0, 0), (64, 63));
    let mut tw = TimeWidget::new((1, 1), (62, 8));

    loop {
        canvas.clear();

        let date_font =
            LedFont::new(Path::new("../fonts/5x7.bdf")).expect("Failed to load date font");

        lw.render(&mut canvas);
        tw.render(&mut canvas);

        let now = Utc::now().with_timezone(&Local);
        let days_until = end_date
            .signed_duration_since(now.naive_local().date())
            .num_days();
        let progress = now
            .naive_local()
            .date()
            .signed_duration_since(start_date)
            .num_days() as f64
            / end_date.signed_duration_since(start_date).num_days() as f64;

        // Draw the days remaining
        canvas.draw_text(
            &date_font,
            &format!("{}", days_until),
            48,
            16,
            &colors["daysrm"],
            0,
            false,
        );

        // Display the loading bar
        canvas.draw_line(2, 11, 45, 11, &colors["line"]);
        canvas.draw_line(2, 14, 45, 14, &colors["line"]);
        canvas.draw_line(2, 12, 2, 13, &colors["line"]);
        canvas.draw_line(45, 12, 45, 13, &colors["line"]);
        let line_progress = 3 + ((44 - 3) as f64 * f64::min(1., progress)) as i32;
        canvas.draw_line(3, 12, line_progress as i32, 12, &colors["daysrm"]);
        canvas.draw_line(3, 13, line_progress as i32, 13, &colors["daysrm"]);

        canvas = matrix.swap(canvas);
    }
}
