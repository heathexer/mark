#[macro_use(azip)]
extern crate ndarray;

use chrono::prelude::*;
use countdown::CountdownWidget;
use life::LifeWidget;
use rpi_led_matrix::{LedColor, LedFont, LedMatrix, LedMatrixOptions, LedRuntimeOptions};
use time::TimeWidget;

pub mod countdown;
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

    let matrix = LedMatrix::new(Some(options), Some(rt_options)).unwrap();
    let mut canvas = matrix.offscreen_canvas();

    let mut lw = LifeWidget::new((0, 0), (64, 63));
    let mut tw = TimeWidget::new((1, 1), (62, 7));
    let mut cw = CountdownWidget::new((1, 8), (62, 7));

    let mut loopcount: u32 = 0;

    loop {
        canvas.clear();

        lw.render(&mut canvas);
        tw.render(&mut canvas);
        cw.render(&mut canvas);

        canvas = matrix.swap(canvas);
        loopcount += 1;
    }
}
