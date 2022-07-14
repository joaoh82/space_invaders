use bevy::{prelude::*, core::FixedTimestep};
use rand::Rng;
use crate::{GameTextures, SPRITE_SCALE, WinSize, components::{Enemy, SpriteSize}, ENEMY_SIZE, EnemyCount, MAX_ENEMY_COUNT};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(1.))
                .with_system(enemy_spawn_system),
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