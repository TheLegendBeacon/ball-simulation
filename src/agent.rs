use rand::{self, Rng};
use raylib::prelude::*;

#[derive(Clone)]
pub struct Agent {
    pub pos: [f64; 2],
    pub angle: f64,
    pub past: Vec<[f64; 2]>,
}

impl Agent {
    pub fn calc_move(&mut self, speed: u8, time: u8, size: (u16, u16)) -> () {
        if !(0.0 <= self.pos[0] && self.pos[0] <= (size.0 as f64))
            || !(0.0 <= self.pos[1] && self.pos[1] <= (size.1 as f64))
        {
            self.angle = rand::thread_rng().gen_range(0..180) as f64;
        }

        self.past.push(self.pos.clone());

        self.pos[0] = self.pos[0] + self.angle.cos() * (speed * time) as f64;
        self.pos[1] = self.pos[1] + self.angle.sin() * (speed * time) as f64;
    }
}

pub struct Screen {
    pub agents: Vec<Agent>,
    pub size: (u16, u16),
}

impl Screen {
    pub fn draw(&mut self, drawhandle: &mut RaylibDrawHandle) -> () {
        for agent in self.agents.as_mut_slice() {
            drawhandle.draw_circle_v(
                Vector2 {
                    x: (agent.pos[0] as f32),
                    y: (agent.pos[1] as f32),
                },
                2.0,
                Color::WHITE,
            );

            while &agent.past.len() > &30 {
                agent.past.remove(0);
            }

            for point in &agent.past {
                drawhandle.draw_circle_v(
                    Vector2 {
                        x: point[0] as f32,
                        y: point[1] as f32,
                    },
                    2.0,
                    Color::new(255, 255, 255, 255),
                )
            }
        }
    }

    pub fn step(&mut self, drawhandle: &mut RaylibDrawHandle, time: u8, speed: u8) {
        for agent in self.agents.iter_mut() {
            agent.calc_move(speed, time, self.size)
        }
        self.draw(drawhandle);
    }
}
