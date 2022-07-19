#![allow(unused)]
mod player;
mod components;
mod enemy;

use bevy::{prelude::*, math::Vec3Swizzles, sprite::collide_aabb::collide, ecs::{system::Insert, schedule::ShouldRun}, utils::HashSet, core::FixedTimestep};
use bevy_kira_audio::{Audio, AudioPlugin, AudioSource};
use components::{Velocity, Player, Movable, SpriteSize, Laser, FromPlayer, Enemy, ExplosionToSpawn, Explosion, ExplosionTimer, FromEnemy, Attributes, HealthText};
use enemy::EnemyPlugin;
use player::*;

// region: --- Asset Constants

const PLAYER_SPRITE: &str = "player_b_01.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_LASER_SPRITE: &str = "laser_a_01.png";
const PLAYER_LASER_SIZE: (f32, f32) = (9., 54.);
const PLAYER_LASER_SOUND: &str = "sounds/player_laser.wav";

const ENEMY_SPRITE: &str = "enemy_a_01.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_LASER_SPRITE: &str = "laser_b_01.png";
const ENEMY_LASER_SIZE: (f32, f32) = (17., 55.);
const ENEMY_LASER_SOUND: &str = "sounds/enemy_laser.wav";

const EXPLOSION_SHEET: &str = "explo_a_sheet.png";
const ENEMY_EXPLOSION_SOUND: &str = "sounds/explosion.wav";
const PLAYER_EXPLOSION_SOUND: &str = "sounds/player_explosion.wav";
const PLAYER_HIT_SOUND: &str = "sounds/player_hit.ogg";

const SPRITE_SCALE: f32 = 0.5;

const EXPLOSION_LENGTH: usize = 16;

// endregion: --- Asset Constants

// region: --- Game Contants

const TIME_STEP: f32 = 1. / 60.;
const BASE_SPEED: f32 = 500.;

const PLAYER_RESPAWN_DELAY: f64 = 2.;
const MAX_ENEMY_COUNT: u32 = 10;
const FORMATION_MEMBERS_MAX: u32 = 2;

// endregion: --- Game Contants

// region: --- Resources

pub struct WinSize {
    pub width: f32,
    pub height: f32,
}

struct GameTextures {
    player: Handle<Image>,
    player_laser: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
    explosion: Handle<TextureAtlas>,
}

struct GameSounds {
    player_laser: Handle<AudioSource>,
    enemy_laser: Handle<AudioSource>,
    enemy_explosion: Handle<AudioSource>,
    player_explosion: Handle<AudioSource>,
    player_hit: Handle<AudioSource>,
}

struct EnemyCount(u32);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    MainMenu,
    InGame,
    Paused,
}

struct PlayerState {
    on: bool, // is the player alive?
    last_shot: f64, // -1 if not shot
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            on: false,
            last_shot: -1.,
        }
    }
}

pub struct GameState {
    pub score: u32,
}

impl Default for GameState {
    fn default() -> Self {
        GameState {
            score: 0,
        }
    }
}

impl PlayerState {
    pub fn is_alive(&self) -> bool {
        self.on
    }

    pub fn shot(&mut self, time: f64) {
        self.on = false;
        self.last_shot = time;
    }

    pub fn spawned(&mut self) {
        self.on = true;
        self.last_shot = -1.;
    }
}
    
// endregion: --- Resources

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(WindowDescriptor {
            title: "Space Invaders".to_string(),
            width: 598.0,
            height: 676.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system) // Called once at the beginning of the game
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_plugin(AudioPlugin)
        .add_state(AppState::InGame)
        .add_system_set(
            SystemSet::on_update(AppState::InGame) // Only run this system when in the InGame state                  
                .with_system(movable_system)
                .with_system(player_laser_hit_enemy_system)
                .with_system(enemy_laser_hit_player_system)
                .with_system(explosion_to_spawn_system)
                .with_system(explostion_animation_system)
        )
        .add_system(main_keyboard_input_system)
        .run();
}

fn setup_system(
    mut commands: Commands, 
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default()); // this is needed to see the ui

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

    // Create explosion texture atlas
    let texture_handle = asset_server.load(EXPLOSION_SHEET);
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(64., 64.), 4, 4);
    let explosion = texture_atlases.add(texture_atlas);

    // add GameTextures resource
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_laser: asset_server.load(PLAYER_LASER_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_LASER_SPRITE),
        explosion: explosion,
    };

    // add GameSounds resource
    let game_sounds = GameSounds {
        player_laser: asset_server.load(PLAYER_LASER_SOUND),
        enemy_laser: asset_server.load(ENEMY_LASER_SOUND),
        enemy_explosion: asset_server.load(ENEMY_EXPLOSION_SOUND),
        player_explosion: asset_server.load(PLAYER_EXPLOSION_SOUND),
        player_hit: asset_server.load(PLAYER_HIT_SOUND),
    };

    commands.insert_resource(game_textures);
    commands.insert_resource(game_sounds);
    commands.insert_resource(EnemyCount(0));
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

fn player_laser_hit_enemy_system(
    mut commands: Commands,
    mut enemy_count: ResMut<EnemyCount>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromPlayer>)>,
    enemy_query: Query<(Entity, &Transform, &SpriteSize), With<Enemy>>,
    mut game_state: ResMut<GameState>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
) {
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate over lasers
    for (laser_entity, laser_tf, laser_size) in laser_query.iter() {
        if despawned_entities.contains(&laser_entity) {
            continue;
        }
        let laser_scale = Vec2::from(laser_tf.scale.xy());

        // iterate over enemies
        for (enemy_entity, enemy_tf, enemy_size) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity) || despawned_entities.contains(&laser_entity) {
                continue;
            }

            let enemy_scale = Vec2::from(enemy_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                enemy_tf.translation,
                enemy_size.0 * enemy_scale,
            );

            // perform collision action
            if let Some(collision) = collision {
                // remove laser
                commands.entity(laser_entity).despawn();
                despawned_entities.insert(laser_entity);

                // remove enemy
                commands.entity(enemy_entity).despawn();
                despawned_entities.insert(enemy_entity);
                enemy_count.0 -= 1;

                // spawn explosion
                commands.spawn().insert(ExplosionToSpawn(enemy_tf.translation.clone()));
                // Playing the explosion sound
                audio.play(game_sounds.enemy_explosion.clone());
                // Updating game state - score
                game_state.score += 5;

            }
        }
    }
}

fn enemy_laser_hit_player_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    time: Res<Time>,
    laser_query: Query<(Entity, &Transform, &SpriteSize), (With<Laser>, With<FromEnemy>)>,
    mut player_query: Query<(Entity, &Transform, &SpriteSize, &mut Attributes), With<Player>>,
    game_sounds: Res<GameSounds>,
    audio: Res<Audio>,
){
    if let Ok((player_entity, player_tf, player_size, mut player_attributes)) = player_query.get_single_mut() {
        let player_scale = Vec2::from(player_tf.scale.xy());

        for (laser_entidy, laser_tf, laser_size) in laser_query.iter() {
            let laser_scale = Vec2::from(laser_tf.scale.xy());

            // determine if collision
            let collision = collide(
                laser_tf.translation,
                laser_size.0 * laser_scale,
                player_tf.translation,
                player_size.0 * player_scale,
            );

            // perform collision action
            if let Some(collision) = collision {
                // take damage
                player_attributes.health -= 10.;

                //println!("Player health: {}", player_attributes.health);

                // remove laser
                commands.entity(laser_entidy).despawn();

                // If players health is 0, spawn explosion and despawn player
                if player_attributes.health <= 0. {
                    // remove player
                    commands.entity(player_entity).despawn();
                    player_state.shot(time.seconds_since_startup());

                    // spawn explosion
                    commands.spawn().insert(ExplosionToSpawn(player_tf.translation.clone()));

                    // Playing the explosion sound
                    audio.play(game_sounds.player_explosion.clone());
                }else{
                    // Playing the hit sound
                    audio.play(game_sounds.player_hit.clone());
                }

                // Always break if there is a collision
                break;
            }
        }
    }
}

fn explosion_to_spawn_system(
    mut commands: Commands,
    query: Query<(Entity, &ExplosionToSpawn)>,
    game_textures: Res<GameTextures>,
) {
    for (explosion_spawn_entity, explostion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands.spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.explosion.clone(),
            transform: Transform {
                translation: explostion_to_spawn.0,
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Explosion)
        .insert(ExplosionTimer::default());

        // despawn the explostion to spawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

fn explostion_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut ExplosionTimer, &mut TextureAtlasSprite), With<Explosion>>,
) {
    for (entity, mut timer, mut sprite) in query.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1;
            if sprite.index >= EXPLOSION_LENGTH {
                commands.entity(entity).despawn();
            }
        }
    }
}

fn main_keyboard_input_system(
    mut commands: Commands,
    mut app_state: ResMut<State<AppState>>,
    keyboard: Res<Input<KeyCode>>,
) {
    // keyboard.just_pressed limits the press to only one time instead of a series of presses
    if keyboard.just_pressed(KeyCode::P) {
        match app_state.current() {
            AppState::MainMenu => {
                println!("Does nothing");
            }
            AppState::InGame => {
                app_state.set(AppState::Paused).unwrap();
                println!("paused");
            }
            AppState::Paused => {
                app_state.set(AppState::InGame).unwrap();
                println!("unpaused");
            }
        }
    }
}