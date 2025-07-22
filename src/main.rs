use macroquad::prelude::*;
use std::io::prelude::*;

fn win_conf() -> macroquad::window::Conf {
    Conf {
        window_title: "Simutlation".to_string(),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

#[macroquad::main(win_conf())]
async fn main() {
    let mut particle = Point {
        origin: Vec2::new(screen_width() / 2., screen_height() / 2.),
        vel: Vec2::ZERO,
    };
    let mut particle1 = Point {
        origin: Vec2::new(1., 1.),
        vel: Vec2::ZERO,
    };

    loop {
        clear_background(BLUE);
        particle.update();
        particle1.update();
        particle1.draw();
        particle.draw();
        next_frame().await
    }
}

struct Point {
    origin: Vec2,
    vel: Vec2,
}

impl Point {
    pub fn update(&mut self) {
        self.origin += self.vel;
    }
    pub fn draw(&self) {
        draw_circle(self.origin.x, self.origin.y, 5., RED);
    }
}
