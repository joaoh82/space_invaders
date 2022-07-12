#![allow(unused)]
mod player;
mod components;

use bevy::prelude::*;
use components::{Velocity, Player, Movable};
use player::*;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_a_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);

const SPRITE_SCALE: f32 = 0.5;

// endregion: --- Asset Constants

// region: --- Game Contants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

// endregion: --- Game Contants

// region: --- Resources

pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
}

// endregion: --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_startup_system(setup_system) // Called once at the beginning of the game
        .add_system(movable_system)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // capture window size
    let window = windows.get_primary_mut().unwrap();
    let (window_width, window_height) = (window.width(), window.height());

    // position window (for tutorial)
    window.set_position(IVec2::new(0, 0));
    // window.set_maximized(true);

    // Setting window size
    let win_size = WinSize {
        width: window_width,
        height: window_height,
    };
    commands.insert_resource(win_size);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
    };
    commands.insert_resource(game_textures);

}

fn movable_system(
    mut commands: Commands,
	win_size: Res<WinSize>,
	mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>,
) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;

        if movable.auto_despawn {
            // despawn if offscreen
            const MARGIN: f32 = 200.;
            if translation.y > win_size.height / 2. + MARGIN 
                || translation.y < -win_size.height / 2. - MARGIN 
                || translation.x > win_size.width / 2. + MARGIN 
                || translation.x < -win_size.width / 2. - MARGIN 
            {
                commands.entity(entity).despawn();
            }
        }
    }
}