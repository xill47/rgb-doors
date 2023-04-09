use crate::{player::Player, ui::RootUI};
use bevy::{prelude::*, render::camera::OrthographicProjection};
use bevy_ecs_ldtk::*;

#[allow(clippy::type_complexity)]
pub fn camera_fit_inside_current_level(
    mut camera_query: Query<(&mut OrthographicProjection, &mut Transform), Without<Player>>,
    game_screen_q: Query<&Node, (With<RootUI>, Changed<Node>)>,
    level_query: Query<&Handle<LdtkLevel>, Without<OrthographicProjection>>,
    ldtk_levels: Res<Assets<LdtkLevel>>,
) {
    let Ok(game_screen) = game_screen_q.get_single() else { return; };
    let screen_size = game_screen.size();
    let aspect_ratio = screen_size.x / screen_size.y;
    let (mut orthographic_projection, mut camera_transform) = camera_query.single_mut();

    for level_handle in &level_query {
        if let Some(ldtk_level) = ldtk_levels.get(level_handle) {
            let level = &ldtk_level.level;
            let level_ratio = level.px_wid as f32 / level.px_hei as f32;
            let (width, height, offset_x, offset_y) = if level_ratio > aspect_ratio {
                info!("level is wider than the screen");
                let width = level.px_wid as f32;
                let height = width / aspect_ratio;
                let offset_y = (level.px_hei as f32 - height) * 0.5;
                (width, height, 0., offset_y)
            } else {
                info!("level is taller than the screen");
                let height = level.px_hei as f32;
                let width = height * aspect_ratio;
                let offset_x = (level.px_wid as f32 - width) * 0.5;
                (width, height, offset_x, 0.)
            };

            orthographic_projection.viewport_origin = Vec2::ZERO;
            orthographic_projection.scaling_mode =
                bevy::render::camera::ScalingMode::Fixed { width, height };
            camera_transform.translation.x = offset_x;
            camera_transform.translation.y = offset_y;
        }
    }
}
