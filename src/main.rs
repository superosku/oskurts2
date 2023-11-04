use minifb::{Key, MouseMode, Window, WindowOptions};
use rand::Rng;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};
use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::game::Game;
use crate::game_thing::GameThing;

mod camera;
mod entity;
mod game;
mod game_thing;
mod ground;
mod vec;

fn main() {
    println!("Hello, world!");

    let mut window = Window::new(
        "Rts2",
        camera::SCREEN_WIDTH,
        camera::SCREEN_HEIGHT,
        WindowOptions {
            ..WindowOptions::default()
        },
    )
    .unwrap();

    let mut camera = camera::Camera::new();

    let size = window.get_size();
    let mut dt = DrawTarget::new(size.0 as i32, size.1 as i32);

    let mut game = Game::new();

    let target_fps = 60;
    let frame_duration = Duration::from_secs(1) / target_fps as u32;

    let mut last_frame_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let frame_time = now - last_frame_time;

        if frame_time >= frame_duration {
            last_frame_time = now;
        } else {
            continue;
            // std::thread::sleep(frame_duration - frame_time);
        }
        // Update window title with the FPS
        window.set_title(&format!("Rts2 - FPS: {}", 1.0 / frame_time.as_secs_f32()));

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        ));

        game.update();
        game.draw(&mut dt, &camera);

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}
