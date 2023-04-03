use bevy::prelude::*;

#[derive(Component)]
pub struct BackgroundColorTween {
    pub start_color: Color,
    pub end_color: Color,
    pub duration: f32,
    pub elapsed: f32,
}

pub fn tween_background_color(
    mut commands: Commands,
    mut query: Query<(&mut BackgroundColor, &mut BackgroundColorTween, Entity)>,
    time: Res<Time>,
) {
    for (mut background_color, mut background_color_tween, entity) in query.iter_mut() {
        background_color_tween.elapsed += time.delta_seconds();
        let t = background_color_tween.elapsed / background_color_tween.duration;
        if t >= 1.0 {
            background_color.0 = background_color_tween.end_color;
            commands.entity(entity).remove::<BackgroundColorTween>();
        } else {
            let start_color_as_vec4 =
                Vec4::from_array(background_color_tween.start_color.as_rgba_f32());
            let end_color_as_vec4 =
                Vec4::from_array(background_color_tween.end_color.as_rgba_f32());
            let color_as_vec4 = start_color_as_vec4.lerp(end_color_as_vec4, t);
            background_color.0 = color_as_vec4.into();
        }
    }
}
