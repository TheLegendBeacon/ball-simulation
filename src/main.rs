use std::env;
use raylib::prelude::*;
use rand::Rng;
mod agent;

use agent::*;

const HEIGHT: u16 = 480;
const WIDTH: u16 = 640;

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
    let mut ball_vec = Vec::new();
    for _ in 0..balls {
        ball_vec.push(Agent {
            pos: [(WIDTH/2) as f64, (HEIGHT/2) as f64],
            angle: rand::thread_rng().gen_range(0..360) as f64,
            past: Vec::new()
        })
    }
    let (mut rl, thread) = raylib::init().size(WIDTH.into(), HEIGHT.into()).title("Ball Pit").build();
    let mut screen = Screen {
        agents: ball_vec,
        size: (WIDTH, HEIGHT),
    };

    rl.set_target_fps(30);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::BLACK);
        screen.step(&mut d, 1, 4);
    }
}
