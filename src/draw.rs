use crate::camera::Camera;
use crate::vec::Vec2f;
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source};

pub fn draw_health_bar(
    dt: &mut DrawTarget,
    camera: &Camera,
    health_ratio: f32,
    health_bar_top: &Vec2f,
    health_bar_width: f32,
) {
    let health_bar_top_left = camera.world_to_screen(&Vec2f::new(
        health_bar_top.x - health_bar_width / 2.0,
        health_bar_top.y,
    ));
    let health_bar_bottom_right = camera.world_to_screen(&Vec2f::new(
        health_bar_top.x + health_bar_width / 2.0,
        health_bar_top.y + 0.1,
    ));
    let health_bar_mid_x =
        health_bar_top_left.x + (health_bar_bottom_right.x - health_bar_top_left.x) * health_ratio;

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(health_bar_top_left.x, health_bar_top_left.y);
    path_builder.line_to(health_bar_mid_x, health_bar_top_left.y);
    path_builder.line_to(health_bar_mid_x, health_bar_bottom_right.y);
    path_builder.line_to(health_bar_top_left.x, health_bar_bottom_right.y);
    path_builder.close();

    let green = (health_ratio * 255.0) as u8;
    let red = ((1.0 - health_ratio) * 255.0) as u8;

    dt.fill(
        &path_builder.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            255, red, green, 0x00,
        )),
        &DrawOptions::new(),
    );

    let mut path_builder = PathBuilder::new();
    path_builder.move_to(health_bar_mid_x, health_bar_top_left.y);
    path_builder.line_to(health_bar_bottom_right.x, health_bar_top_left.y);
    path_builder.line_to(health_bar_bottom_right.x, health_bar_bottom_right.y);
    path_builder.line_to(health_bar_mid_x, health_bar_bottom_right.y);
    path_builder.close();

    dt.fill(
        &path_builder.finish(),
        &Source::Solid(SolidSource::from_unpremultiplied_argb(
            255, 0x00, 0x00, 0x00,
        )),
        &DrawOptions::new(),
    );
}
