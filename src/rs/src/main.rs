use rpi-led-matrix::{LedColor, LedMatrix};

fn main() {
    let matrix = LedMatrix::new(None, None).expect("Failed to create LedMatrix"));
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
