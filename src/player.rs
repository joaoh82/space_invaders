use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, components::{Velocity, Player, Movable}, TIME_STEP, BASE_SPEED};
use bevy::{prelude::*, input::keyboard};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        .add_system(player_keyboard_event_system)
        .add_system(player_fire_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut win_size: ResMut<WinSize>,
) {
    // add player
    let bottom = -win_size.height / 2.;
    commands.spawn_bundle(SpriteBundle {
        texture: game_textures.player.clone(),
        transform: Transform {
            translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5., 10.0),
            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
            ..Default::default()
        },
        ..Default::default()
    })
    .insert(Player)
    .insert(Movable{auto_despawn: false})
    .insert(Velocity{x: 0., y: 0.}); // Initial velocity of player
}

fn player_fire_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut keyboard: Res<Input<KeyCode>>,
    query: Query<&Transform, With<Player>>,
) {
   if let Ok(player_tf) = query.get_single() {
    if keyboard.just_pressed(KeyCode::Space) {
        let (x, y) = (player_tf.translation.x, player_tf.translation.y);
        let x_offset = PLAYER_SIZE.0 / 2. * SPRITE_SCALE - 5.;

        // Creating a closure to spawn a laser
        let mut spawn_laser = |x_offset: f32| {
            commands.spawn_bundle(SpriteBundle {
                texture: game_textures.player_laser.clone(),
                transform: Transform {
                    translation: Vec3::new(x + x_offset, y + 15., 0.0),
                    scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Movable{auto_despawn: true})
            .insert(Velocity{x: 0., y: 1.});
        };

        // Spawning the lasers - one on the left and one on the right
        spawn_laser(-x_offset);
        spawn_laser(x_offset);
    }
   }
}

fn player_keyboard_event_system(
    keyboard: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity, With<Player>>
) {
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x = if keyboard.pressed(KeyCode::A) {
            -1.
        } else if keyboard.pressed(KeyCode::D) {
            1.
        } else {
            0.
        }
    }
}