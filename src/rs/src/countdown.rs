use crate::config::CountdownOptions;
use chrono::prelude::*;
use rpi_led_matrix::{LedCanvas, LedColor, LedFont};
use std::path::Path;

pub struct CountdownWidget {
    position: (usize, usize),
    size: (usize, usize),
    line_color: LedColor,
    main_color: LedColor,
    font: LedFont,
    start_date: NaiveDate,
    end_date: NaiveDate,
}

impl CountdownWidget {
    pub fn new(position: (usize, usize), size: (usize, usize), config: CountdownOptions) -> Self {
        CountdownWidget {
            position: position,
            size: size,
            line_color: config.line_color.into(),
            main_color: config.main_color.into(),
            font: LedFont::new(Path::new(&config.font_path)).expect("Failed to load font"),
            start_date: NaiveDate::from_ymd(2022, 3, 28),
            end_date: NaiveDate::from_ymd(2022, 6, 9),
        }
    }

    pub fn render(&self, canvas: &mut LedCanvas) {
        let now = Utc::now().with_timezone(&Local);
        let days_until = self
            .end_date
            .signed_duration_since(now.naive_local().date())
            .num_days();
        let days_until = format!("{}", days_until);
        let progress = now
            .naive_local()
            .date()
            .signed_duration_since(self.start_date)
            .num_days() as f64
            / self
                .end_date
                .signed_duration_since(self.start_date)
                .num_days() as f64;

        let (px, py) = (self.position.0 as i32, self.position.1 as i32);
        let (sx, sy) = (self.size.0 as i32, self.size.1 as i32);
        let tw = days_until.len() as i32 * 5;

        // Draw the days remaining
        canvas.draw_text(
            &self.font,
            &days_until,
            px + sx - tw + 1,
            py + sy,
            &self.main_color,
            0,
            false,
        );

        // Display the loading bar
        canvas.draw_line(
            px,
            py + sy - 2,
            px + sx - tw - 2,
            py + sy - 2,
            &self.line_color,
        );
        canvas.draw_line(
            px,
            py + sy - 5,
            px + sx - tw - 2,
            py + sy - 5,
            &self.line_color,
        );
        canvas.draw_line(
            px,
            py + sy - 2________,
            px,
            py + sy - 4________,
            &self.line_color,
        );
        canvas.draw_line(
            px + sx - tw - 2,
            py + sy - 2,
            px + sx - tw - 2,
            py + sy - 4,
            &self.line_color,
        );

        let line_progress = px + 2 + ((px + sx - tw - 5) as f64 * f64::min(0., progress)) as i32;
        canvas.draw_line(
            px + 1,
            py + sy - 3,
            line_progress,
            py + sy - 3,
            &self.main_color,
        );
        canvas.draw_line(
            px + 1,
            py + sy - 4,
            line_progress,
            py + sy - 4,
            &self.main_color,
        );
    }
}
