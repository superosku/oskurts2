use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use rand::Rng;
use raqote::{
    DrawOptions, DrawTarget, LineCap, LineJoin, PathBuilder, SolidSource, Source, StrokeStyle,
};
use rayon::prelude::*;
use std::os::unix::raw::uid_t;
use std::time::{Duration, Instant};

use crate::game::Game;
use crate::game_thing::GameThing;
use crate::vec::Vec2f;

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
    let mut last_second_fpses: Vec<f32> = Vec::new();

    let mut drag_start_pos: Option<Vec2f> = None;

    let mut selected_ids: Vec<usize> = Vec::new();

    let mut right_button_pressed = false;
    let mut right_button_down = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();
        let frame_time = now - last_frame_time;

        if frame_time >= frame_duration {
            last_frame_time = now;
        } else {
            continue;
            // std::thread::sleep(frame_duration - frame_time);
        }

        dt.clear(SolidSource::from_unpremultiplied_argb(
            0xff, 0x00, 0x00, 0x00,
        ));

        if window.is_key_down(Key::A) {
            camera.move_position(&Vec2f::new(-1.0, 0.0));
        }
        if window.is_key_down(Key::D) {
            camera.move_position(&Vec2f::new(1.0, 0.0));
        }
        if window.is_key_down(Key::W) {
            camera.move_position(&Vec2f::new(0.0, -1.0));
        }
        if window.is_key_down(Key::S) {
            camera.move_position(&Vec2f::new(0.0, 1.0));
        }

        match window.get_scroll_wheel() {
            Some((_, y_scroll)) => {
                println!("Scroll: {}", y_scroll);
                if y_scroll != 0.0 {
                    camera.zoom(1.0 + y_scroll as f32 * 0.01);
                }
            }
            _ => {}
        }

        let current_fps = 1.0 / frame_time.as_secs_f32();
        last_second_fpses.push(current_fps);
        if last_second_fpses.len() > 60 {
            last_second_fpses.remove(0);
        }
        let average_fps = last_second_fpses.iter().sum::<f32>() / last_second_fpses.len() as f32;
        window.set_title(&format!("Rts2 - FPS: {}", average_fps));

        game.update();
        game.draw(&mut dt, &camera, &selected_ids);

        let mouse_pos = window.get_mouse_pos(MouseMode::Clamp).unwrap();
        let mouse_pos_game = camera.screen_to_world(&Vec2f::new(mouse_pos.0, mouse_pos.1));
        let left_button_down = window.get_mouse_down(MouseButton::Left);

        let new_right_button_down = window.get_mouse_down(MouseButton::Right);
        if new_right_button_down && !right_button_down {
            right_button_pressed = true;
        } else {
            right_button_pressed = false;
        }
        right_button_down = new_right_button_down;

        if right_button_pressed {
            println!("Commading a move");
            game.command_entities_move(&selected_ids, &mouse_pos_game);
        }

        if left_button_down {
            match &drag_start_pos {
                Some(start_pos) => {
                    let mut screen_start_pos = camera.world_to_screen(&start_pos);
                    let mut screen_end_pos = camera.world_to_screen(&mouse_pos_game);

                    // Draw a rectangle from drag_start_pos to mouse_pos_game
                    let mut path_builder = PathBuilder::new();
                    path_builder.move_to(screen_start_pos.x, screen_start_pos.y);
                    path_builder.line_to(screen_end_pos.x, screen_start_pos.y);
                    path_builder.line_to(screen_end_pos.x, screen_end_pos.y);
                    path_builder.line_to(screen_start_pos.x, screen_end_pos.y);
                    path_builder.close();

                    let path = path_builder.finish();

                    let stroke_style = &mut raqote::StrokeStyle::default();
                    // stroke_style.width = camera.length_to_pixels(0.1);
                    dt.fill(
                        &path,
                        &Source::Solid(SolidSource::from_unpremultiplied_argb(
                            0x80, 0xff, 0xff, 0xff,
                        )),
                        // stroke_style,
                        &DrawOptions::new(),
                    );
                }
                None => {
                    drag_start_pos = Some(mouse_pos_game);
                }
            }
        } else {
            match &drag_start_pos {
                Some(start_pos) => {
                    selected_ids = game.entity_ids_in_bounding_box(
                        Vec2f::new(
                            start_pos.x.min(mouse_pos_game.x),
                            start_pos.y.min(mouse_pos_game.y),
                        ),
                        Vec2f::new(
                            start_pos.x.max(mouse_pos_game.x),
                            start_pos.y.max(mouse_pos_game.y),
                        ),
                    );
                    drag_start_pos = None;
                }
                None => {}
            }
        }

        window
            .update_with_buffer(dt.get_data(), size.0, size.1)
            .unwrap();
    }
}
