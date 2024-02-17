//! This example shows how to use 2d lighting.

use bevy::prelude::*;

use rand::Rng;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let mut rd = rand::thread_rng();
    for i in 0..3 {
        commands.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(rd.gen(), rd.gen(), rd.gen()),
                custom_size: Some(Vec2::splat(32.0)),
                ..Default::default()
            },
            transform: Transform::from_xyz((i - 1) as f32 * 64.0, 0.0, 0.0),
            ..Default::default()
        });
    }
}
