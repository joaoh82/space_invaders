use std::f32::consts::PI;

use bevy::{prelude::*, core::FixedTimestep, ecs::schedule::ShouldRun};
use rand::{Rng, thread_rng};
use crate::{GameTextures, SPRITE_SCALE, WinSize, components::{Enemy, SpriteSize, Movable, FromEnemy, Laser, Velocity}, ENEMY_SIZE, EnemyCount, MAX_ENEMY_COUNT, ENEMY_LASER_SIZE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(enemy_fire_criteria)
                .with_system(enemy_fire_system),
        );
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    mut enemy_count: ResMut<EnemyCount>,
    win_size: Res<WinSize>,
) {
    if enemy_count.0 < MAX_ENEMY_COUNT {
        // compute the x and y
        let mut rng = rand::thread_rng();
        let w_span = win_size.width as f32 / 2.0 - 100.0;
        let h_span = win_size.height as f32 / 2.0 - 100.0;
        let x = rng.gen_range(-w_span..w_span);
        let y = rng.gen_range(-h_span..h_span);

        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy.clone(),
            transform: Transform {
                translation: Vec3::new(x, y, 10.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Enemy)
        .insert(SpriteSize::from(ENEMY_SIZE));
        enemy_count.0 += 1;
    }
}

fn enemy_fire_criteria() -> ShouldRun {
    if thread_rng().gen_bool(1. /60.) {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

fn enemy_fire_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    for &enemy_tf in enemy_query.iter() {
        let (x, y) = (enemy_tf.translation.x, enemy_tf.translation.y);
        // spawn enemy laser sprite
        commands.spawn_bundle(SpriteBundle {
            texture: game_textures.enemy_laser.clone(),
            transform: Transform {
                translation: Vec3::new(x, y - 15., 0.),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 0.),
                rotation: Quat::from_rotation_x(PI),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Laser)
        .insert(SpriteSize::from(ENEMY_LASER_SIZE))
        .insert(FromEnemy)
        .insert(Movable { auto_despawn: true })
        .insert(Velocity { x: 0.0, y: -1.0 });
    }
}