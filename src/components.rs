use std::iter::from_fn;

use bevy::{prelude::Component, math::{Vec2, Vec3}, core::Timer};

// region: --- Common Components

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

/// Attribute component is used to store the attributes of an entity (Player or Enemy).
#[derive(Component)]
pub struct Attributes {
    pub health: f32,
}

impl Default for Attributes {
    fn default() -> Self {
        Attributes {
            health: 100.0,
        }
    }
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component)]
pub struct Laser;

#[derive(Component)]
pub struct SpriteSize(pub Vec2);

impl From<(f32, f32)> for SpriteSize {
    fn from(size: (f32, f32)) -> Self {
        SpriteSize(Vec2::new(size.0, size.1))
    }
}

// endregion: --- Common Components

// region: --- UI Components

#[derive(Component)]
pub struct HealthText;

// endregion: --- UI Components

// region: --- Player Components

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct FromPlayer;

// endregion: --- Player Components

// region: --- Enemy Components

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct FromEnemy;

// endregion: --- Enemy Components

// region: --- Explosion Components

#[derive(Component)]
pub struct Explosion;

#[derive(Component)]
pub struct ExplosionToSpawn(pub Vec3);

#[derive(Component)]
pub struct ExplosionTimer(pub Timer);

impl Default for ExplosionTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, true))
    }
}

// endregion: --- Explosion Components
