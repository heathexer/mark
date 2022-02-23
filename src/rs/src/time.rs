use crate::config::TimeOptions;
use chrono::prelude::*;
use rpi_led_matrix::{LedCanvas, LedColor, LedFont};
use std::path::Path;

pub struct TimeWidget {
    position: (usize, usize),
    size: (usize, usize),
    time_color: LedColor,
    month_color: LedColor,
    day_color: LedColor,
    time_font: LedFont,
    date_font: LedFont,
    now: DateTime<Local>,
}

impl TimeWidget {
    pub fn new(position: (usize, usize), size: (usize, usize), config: TimeOptions) -> Self {
        TimeWidget {
            position: position,
            size: size,
            time_color: config.time_color.into(),
            month_color: config.month_color.into(),
            day_color: config.day_color.into(),
            time_font: LedFont::new(Path::new(&config.time_font_path))
                .expect("Failed to load time font"),
            date_font: LedFont::new(Path::new(&config.date_font_path))
                .expect("Failed to load date font"),
            now: Utc::now().with_timezone(&Local),
        }
    }

    pub fn render(&mut self, canvas: &mut LedCanvas) {
        self.now = Utc::now().with_timezone(&Local);
        let time = self.now.format("%-I %M").to_string();
        let month = self.now.format("%b").to_string();
        let day = self.now.format("%-d").to_string();

        // Display the current time
        canvas.draw_text(
            &self.time_font,
            &time,
            self.position.0 as i32,
            self.position.1 as i32 + self.size.1 as i32,
            &self.time_color,
            0,
            false,
        );
        // Blink colon
        if self.now.second() % 2 == 0 {
            canvas.draw_text(
                &self.time_font,
                &":",
                self.position.0 as i32 + 6 * (time.len() as i32 - 3),
                self.position.1 as i32 + self.size.1 as i32 - 1,
                &self.time_color,
                0,
                false,
            );
        }

        // Display the current month and day
        canvas.draw_text(
            &self.date_font,
            &month,
            self.position.0 as i32 + 46 - 5 * day.len() as i32,
            self.position.1 as i32 + self.size.1 as i32 - 1,
            &self.month_color,
            0,
            false,
        );
        canvas.draw_text(
            &self.date_font,
            &day,
            self.position.0 as i32 + 63 - 5 * day.len() as i32,
            self.position.1 as i32 + self.size.1 as i32 - 1,
            &self.day_color,
            0,
            false,
        );
    }
}
