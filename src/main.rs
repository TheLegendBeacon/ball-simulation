use std::env;
use raylib::prelude::*;
mod agent;

use agent::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = str::parse(&args[1]);
    let balls = match parsed {
        Ok(val) => val,
        Err(_) => {
            println!("Please enter a real number");
            return;
        }
    };

    let (mut rl, thread) = raylib::init().size(640, 480).title("Ball Pit").build();
    let mut screen = Screen {
        agents: vec![
            Agent {
                pos: [320.0, 240.0],
                angle: 50.0,
                past: Vec::new()
            };
            balls
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
