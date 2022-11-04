use bevy::prelude::Component;

// # Common Components --- #

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

// # Player Components --- #

#[derive(Component)]
pub struct Player;

