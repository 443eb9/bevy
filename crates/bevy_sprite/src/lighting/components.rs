use bevy_ecs::component::Component;
use bevy_render::color::Color;

#[derive(Component)]
pub struct PointLight {
    pub color: Color,
    pub intensity: f32,
    pub radius: f32,
    pub fall_off: f32,
}
