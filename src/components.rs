use bevy::prelude::Component;

// region: --- Common Components

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

// endregion: --- Common Components

// region: --- Player Components

#[derive(Component)]
pub struct Player;

// endregion: --- Player Components