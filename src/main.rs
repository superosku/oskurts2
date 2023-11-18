use rand::Rng;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};
use rayon::prelude::*;
use std::time::{Duration, Instant};

use crate::game::Game;
use crate::vec::Vec2f;

use crate::constants::{SCREEN_HEIGHT, SCREEN_WIDTH};
use crate::ground::GroundType;
use pixels::{Pixels, SurfaceTexture};
use winit::keyboard::KeyCode;
use winit::{
    dpi::LogicalSize, event::Event, event::WindowEvent, event_loop::EventLoop,
    window::WindowBuilder,
};
use winit_input_helper::WinitInputHelper;

mod building;
mod building_container;
mod camera;
mod constants;
mod entity;
mod entity_container;
mod game;
mod game_thing;
mod ground;
mod path_finder;
mod projectile;
mod projectile_handler;
mod vec;

fn main() {
    println!("Hello, world!");

    let event_loop = EventLoop::new().unwrap();
    let mut input = WinitInputHelper::new();

    let window = {
        let size = LogicalSize::new(SCREEN_WIDTH as f64, SCREEN_HEIGHT as f64);
        let scaled_size = LogicalSize::new(SCREEN_WIDTH as f64 * 1.0, SCREEN_HEIGHT as f64 * 1.0);
        WindowBuilder::new()
            .with_title("Conway's Game of Life")
            .with_inner_size(scaled_size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(SCREEN_WIDTH as u32, SCREEN_HEIGHT as u32, surface_texture).unwrap()
    };

    let mut camera = camera::Camera::new();

    let mut dt = DrawTarget::new(SCREEN_WIDTH as i32, SCREEN_HEIGHT as i32);

    let mut game = Game::new();

    // let target_fps = 60;
    // let frame_duration = Duration::from_secs(1) / target_fps as u32;

    let mut last_frame_time = Instant::now();
    let mut last_second_fpses: Vec<f32> = Vec::new();

    let mut drag_start_pos: Option<Vec2f> = None;
    let mut drag_pos: Option<Vec2f> = None;

    let mut selected_ids: Vec<usize> = Vec::new();
    let mut selected_building_id: Option<usize> = None;

    let mut draw_time = Duration::from_secs(0);
    let mut game_update_time = Duration::from_secs(0);

    event_loop
        .run(move |event, window_target| {
            match &event {
                Event::WindowEvent {
                    window_id,
                    event: window_event,
                } => {
                    match window_event {
                        WindowEvent::CloseRequested => {
                            window_target.exit();
                        }
                        WindowEvent::RedrawRequested => {
                            let draw_start = Instant::now();

                            dt.clear(SolidSource::from_unpremultiplied_argb(
                                0xff, 0x00, 0x00, 0x00,
                            ));
                            game.draw(&mut dt, &camera, &selected_ids, &selected_building_id);

                            match (&drag_start_pos, &drag_pos) {
                                (Some(pos1), Some(pos2)) => {
                                    let mut screen_start_pos = camera.world_to_screen(&pos1);
                                    let mut screen_end_pos = camera.world_to_screen(&pos2);

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
                                _ => {}
                            }

                            for (dst, &src) in pixels
                                .frame_mut()
                                .chunks_exact_mut(4)
                                .zip(dt.get_data().iter())
                            {
                                dst[0] = (src >> 16) as u8;
                                dst[1] = (src >> 8) as u8;
                                dst[2] = src as u8;
                                dst[3] = (src >> 24) as u8;
                            }

                            pixels.render().unwrap();

                            draw_time = Instant::now() - draw_start;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }

            if input.update(&event) {
                let now = Instant::now();
                let frame_time = now - last_frame_time;
                last_frame_time = now;

                let current_fps = 1.0 / frame_time.as_secs_f32();
                last_second_fpses.push(current_fps);
                if last_second_fpses.len() > 60 {
                    last_second_fpses.remove(0);
                }
                let average_fps =
                    last_second_fpses.iter().sum::<f32>() / last_second_fpses.len() as f32;
                window.set_title(&format!("Rts2 - FPS: {}", average_fps));

                if input.key_held(KeyCode::KeyA) {
                    camera.move_position(&Vec2f::new(-1.0, 0.0));
                }
                if input.key_held(KeyCode::KeyD) {
                    camera.move_position(&Vec2f::new(1.0, 0.0));
                }
                if input.key_held(KeyCode::KeyW) {
                    camera.move_position(&Vec2f::new(0.0, -1.0));
                }
                if input.key_held(KeyCode::KeyS) {
                    camera.move_position(&Vec2f::new(0.0, 1.0));
                }

                if input.key_pressed(KeyCode::KeyU) {
                    if let Some(debug_path) = &game.debug_path {
                        debug_path.borrow_mut().do_orienting_round();
                    }
                }

                let scroll_diff = input.scroll_diff();
                if scroll_diff.1 != 0.0 {
                    camera.zoom(1.0 + scroll_diff.1 / 100.0);
                }

                if input.key_pressed(KeyCode::KeyH) {
                    game.command_entities_simple(&selected_ids, false, true);
                }

                if input.key_pressed(KeyCode::KeyS) {
                    game.command_entities_simple(&selected_ids, true, false);
                }

                let cursor_option = input.cursor();
                if let Some(cursor) = cursor_option {
                    let scale = window.scale_factor() as f32;
                    let cursor_game_pos =
                        camera.screen_to_world(&Vec2f::new(cursor.0 / scale, cursor.1 / scale));

                    if input.key_pressed_os(KeyCode::KeyQ) {
                        if let Some(building_id) = selected_building_id {
                            game.command_building_spawn(building_id);
                            game.command_building_spawn(building_id);
                            game.command_building_spawn(building_id);
                            game.command_building_spawn(building_id);
                            game.command_building_spawn(building_id);
                        }
                    }

                    if input.key_held(KeyCode::KeyJ) {
                        game.ground.set_at(
                            cursor_game_pos.x as i32,
                            cursor_game_pos.y as i32,
                            GroundType::Wall,
                        );
                    }
                    if input.key_held(KeyCode::KeyK) {
                        game.ground.set_at(
                            cursor_game_pos.x as i32,
                            cursor_game_pos.y as i32,
                            GroundType::Empty,
                        );
                    }

                    if input.mouse_pressed(1) || input.key_pressed(KeyCode::KeyR) {
                        game.command_entities_move(
                            &selected_ids,
                            &cursor_game_pos,
                            input.key_held(KeyCode::KeyE),
                        );
                    }
                    if input.mouse_pressed(0) {
                        drag_start_pos = Some(cursor_game_pos.clone());
                        drag_pos = Some(cursor_game_pos.clone());
                    }
                    if input.mouse_held(0) {
                        drag_pos = Some(cursor_game_pos.clone());
                    } else {
                        match (&drag_start_pos, &drag_pos) {
                            (Some(p1), Some(p2)) => {
                                selected_building_id = None;

                                let top_left = Vec2f::new(p1.x.min(p2.x), p1.y.min(p2.y));
                                let bottom_right = Vec2f::new(p1.x.max(p2.x), p1.y.max(p2.y));
                                selected_ids =
                                    game.entity_ids_in_bounding_box(&top_left, &bottom_right);
                                println!("Selected entities: {:?}", selected_ids);
                                if selected_ids.len() == 0 {
                                    let building_id = game
                                        .first_building_id_in_bouding_box(&top_left, &bottom_right);
                                    println!("Selected building: {:?}", building_id);
                                    selected_building_id = building_id;
                                }
                            }
                            _ => {}
                        }

                        drag_start_pos = None;
                        drag_pos = None;
                    }
                }

                let game_update_start = Instant::now();
                game.update();
                game_update_time = Instant::now() - game_update_start;

                window.request_redraw();
            }
        })
        .unwrap();
}
