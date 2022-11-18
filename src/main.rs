use raylib::prelude::*;
mod agent;

use agent::*;

fn main() {
    let (mut rl, thread) = raylib::init().size(640, 480).title("Hello, World").build();
    let mut screen = Screen {
        agents: vec![
            Agent {
                pos: [320.0, 240.0],
                angle: 50.0,
                past: Vec::new()
            };
            50
        ],
        size: (640, 480),
    };

    rl.set_target_fps(30);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        screen.step(&mut d, 1, 4);
    }
}
