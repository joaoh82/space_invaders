use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, PLAYER_LASER_SIZE, components::{Velocity, Player, Movable, FromPlayer, SpriteSize, Laser, Attributes, HealthText}, TIME_STEP, BASE_SPEED, PlayerState, PLAYER_RESPAWN_DELAY, AppState};
use bevy::{prelude::*, input::keyboard, core::FixedTimestep};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerState::default())
            .insert_resource(Attributes::default()) // This is neede to access the attributes from the player
			.add_system_set(
				SystemSet::new()
					.with_run_criteria(FixedTimestep::step(0.5))
					.with_system(player_spawn_system),
			)
            .add_system_set(
                SystemSet::on_update(AppState::InGame)
                .with_system(player_keyboard_event_system)
                .with_system(player_fire_system)
                .with_system(health_text_update_system)
            );
	}
}

/// This system is responsible for updating the value of the player's health.
fn health_text_update_system(
    player_query: Query<(Entity, &Attributes), With<Player>>, 
    mut query: Query<&mut Text, With<HealthText>>
) {
    if let Ok((player_entity, player_attributes)) = player_query.get_single() {
        for mut text in query.iter_mut() {
            let new_health = format!("Health: {}", player_attributes.health);
            // We used the `Text::with_section` helper method, but it is still just a `Text`,
            // so to update it, we are still updating the one and only section
            text.sections[0].value = new_health;
        }
    }
}

fn player_spawn_system(
    mut commands: Commands,
    mut player_state: ResMut<PlayerState>,
    player_attributes: Res<Attributes>, // This is needed to access the attributes from the player
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    mut win_size: ResMut<WinSize>,
    asset_server: Res<AssetServer>, // Need this to spawn the player's health text
) {
    let now = time.seconds_since_startup();
	let last_shot = player_state.last_shot;

	if !player_state.on && (last_shot == -1. || now > last_shot + PLAYER_RESPAWN_DELAY) {
		// add player
		let bottom = -win_size.height / 2.;
		commands
			.spawn_bundle(SpriteBundle {
				texture: game_textures.player.clone(),
				transform: Transform {
					translation: Vec3::new(
						0.,
						bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.,
						10.,
					),
					scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
					..Default::default()
				},
				..Default::default()
			})
			.insert(Player)
			.insert(SpriteSize::from(PLAYER_SIZE))
			.insert(Movable { auto_despawn: false })
			.insert(Velocity { x: 0., y: 0. })
            .insert(Attributes::default());

        // add health text to top left of screen
        commands
            .spawn_bundle(TextBundle {
                style: Style {
                    align_self: AlignSelf::FlexEnd,
                    position_type: PositionType::Absolute,
                    position: Rect {
                        top: Val::Px(15.0),
                        left: Val::Px(15.0),
                        ..default()
                    },
                    ..default()
                },
                // Use the `Text::with_section` constructor
                text: Text::with_section(
                    // Accepts a `String` or any type that converts into a `String`, such as `&str`
                    format!("Health: {}", player_attributes.health.to_string()),
                    
                    TextStyle {
                        font: asset_server.load("fonts/AgentOrange.ttf"),
                        font_size: 16.0,
                        color: Color::GREEN,
                    },
                    Default::default()
                ),
                ..default()
            })
            .insert(HealthText);

		player_state.spawned();
	}
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
            .insert(Laser)
            .insert(FromPlayer)
            .insert(SpriteSize::from(PLAYER_LASER_SIZE))
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