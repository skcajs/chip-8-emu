use chip8::Chip8;
use minifb::{Key, Scale, Window, WindowOptions};

mod chip8;

const WIDTH: usize = 64;
const HEIGHT: usize = 32;

fn main() {

    let cpu = Chip8::new();

    // TODO: Load a ROM

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let opts = WindowOptions {
        resize: true,
        scale: Scale::X4,
        ..Default::default()
    };

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        opts,
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {
        for i in buffer.iter_mut() {
            *i = 0xFFFFFFFF;
        }

        window
            .update_with_buffer(&buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
