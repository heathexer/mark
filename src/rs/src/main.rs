use chrono::prelude::*;
use rpi_led_matrix::{LedColor, LedFont, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use std::path::Path;

fn main() {
    // Options
    let mut options = LedMatrixOptions::new();
    options.set_rows(32);
    options.set_cols(64);
    options.set_chain_length(2);
    options.set_parallel(1);
    options.set_hardware_mapping("adafruit-hat");
    // options.set_pixel_mapper_config("V-mapper");
    options.set_row_addr_type(0);
    options.set_limit_refresh(100);
    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let time_font = LedFont::new(Path::new("../fonts/6x12.bdf")).expect("Failed to load 6x12 font");
    let date_font = LedFont::new(Path::new("../fonts/5x7.bdf")).expect("Failed to load 5x7 font");

    let text_color = LedColor {
        red: 100,
        green: 100,
        blue: 100,
    };
    let bar_color = LedColor {
        red: 0,
        green: 100,
        blue: 0,
    };
    let start_date = NaiveDate::from_ymd(2022, 1, 3);
    let end_date = NaiveDate::from_ymd(2022, 3, 11);

    loop {
        canvas.clear();

        let now = Utc::now().with_timezone(&Local);
        let time = now.format("%-I %M").to_string();
        let month = now.format("%b").to_string();
        let day = now.format("%-d").to_string();

        // Display the current time
        canvas.draw_text(&time_font, &time, 2, 9, &text_color, 0, false);
        // Blink colon
        if (now.second() % 2 == 0) {
            canvas.draw_text(
                &time_font,
                &":",
                2 + 6 * (time.len().try_into().unwrap() - 3),
                8,
                &text_color,
                0,
                false,
            );
        }

        // Display the current month and day
        canvas.draw_text(
            &date_font,
            &month,
            46 - 5 * day.len().try_into().unwrap(),
            8,
            &text_color,
            0,
            false,
        );
        canvas.draw_text(
            &date_font,
            &month,
            63 - 5 * day.len().try_into().unwrap(),
            8,
            &text_color,
            0,
            false,
        );

        canvas = matrix.swap(canvas);
    }
}
