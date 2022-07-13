use bevy::prelude::*;
use rand::Rng;
use crate::{GameTextures, SPRITE_SCALE, WinSize, components::{Enemy, SpriteSize}, ENEMY_SIZE};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, enemy_spawn_system);
        // .add_system(enemy_move_system)
        // .add_system(enemy_despawn_system);
    }
}

fn enemy_spawn_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>,
) {
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
}