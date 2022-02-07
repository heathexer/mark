use rpi_led_matrix::{LedColor, LedMatrix, LedMatrixOptions, LedRuntimeOptions};

fn main() {
    // Options
    let mut options = LedMatrixOptions::new();
    options.set_hardware_mapping("adafruit-hat");
    options.set_pixel_mapper_config("V-mapper");
    options.set_rows(32);
    options.set_cols(64);
    options.set_chain_length(2);
    options.set_parallel(1);
    options.set_row_addr_type(0) // 0 is default but this was in main.py
    options.set_multiplexing(0);
    options.set_limit_refresh(100);
    let mut rt_options = LedRuntimeOptions::new();
    rt_options.set_gpio_slowdown(4);

    let matrix = LedMatrix::new(options, rt_options).expect("Failed to create LedMatrix");
    let mut canvas = matrix.offscreen_canvas();

    for red in (0..255).step_by(16) {
        for green in (0..255).step_by(16) {
            for blue in (0..255).step_by(16) {
                canvas.fill(&LedColor { red, green, blue });
                canvas = matrix.swap(canvas);
            }
        }
    }
}
