use crate::{GameTextures, WinSize, PLAYER_SIZE, PLAYER_SCALE, ARROW_SCALE, TIME_STEP, BASE_SPEED, Velocity, Movable, Player};
use bevy::prelude::*;

pub struct PlayerPlugin;

// This struct PlayerPlugin implements the 'Plugin' trait, which allows it to be imported
// into the main file for use.
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        // the following player_spawn_system (spawns the player sprite) is started AFTER
        // the startup_system (camera, window pos, loads textures), located in the main file:
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        // the following systems are called in every event loop (constant):
        .add_system(player_keyboard_event_system)
        .add_system(player_fire_system);
    }
}

// This system sets up and spawns the player sprite
fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>, // custom Resource, so can access window size from startup system.
) {
    // --- Create sprite entity --- using SpriteBundle (a sprite builder)
    let bottom = -win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            // vvv Within the struct "SpriteBundle", there is a field called "texture", which we'll set
            //     to the handle/pointer/reference to the sprite image, returned from asset_server.load().
            texture: game_textures.player.clone(), // <- Added a New component (property) of sprite (entity).
            // vvv Another component (transforms sprite):
            transform: Transform {
                translation: Vec3::new(
                    0.,
                    (bottom + PLAYER_SIZE.1 / 2. * PLAYER_SCALE + 5.),
                    10.,
                    //^ Default spawn location
                    //  Vec3::new(x, y, z),
                    //  Note: z-axis is the spite layer (layer here is set to 10. for now)
                ),
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.),
                // ^^^ Scales the sprite
                ..Default::default()
            },
            ..Default::default() // use default properties for the rest of the sprite
        }
    )
    .insert(Player)
    .insert(Movable { auto_despawn: false })
    .insert(Velocity {x: 0., y: 0.});
}

// This system sets up and spawns the player arrow sprite
fn player_fire_system(
    mut commands: Commands,
    kb: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    query: Query<&Transform, With<Player>>,
) {
    // if query recieves an Ok enum, set the containing value (&Transform) to new variable
    // 'player_tf'; if Err enum recieved, then do nothing.
    // if Space bar pressed, then set variable x and y to value of &Transform.translation.x and y.
    // Finally, SpriteBundle to create the sprite.
    if let Ok(player_tf) = query.get_single() {
        if kb.just_pressed(KeyCode::Space) {
            let (x, y) = (player_tf.translation.x, player_tf.translation.y);

            commands.spawn_bundle(SpriteBundle{
                texture: game_textures.player_arrow.clone(),
                transform: Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: Vec3::new(ARROW_SCALE, ARROW_SCALE, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Movable { auto_despawn: true })
            .insert(Velocity { x: 0., y: 1. });
        }
    }
}

// keyboard inputs -> Velocity -> Player movement
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<(&mut Velocity), With<Player>>,
) {
    // if query recieves an Ok enum (which contains mut reference to Velocity component),
    // then bind that to new variable 'velocity' and if Left or Right is pressed, set
    // velocity.x to appropriate number.
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.x =
            if kb.pressed(KeyCode::Left) {
                -1.
            } else if kb.pressed(KeyCode::Right) {
                1.
            } else {
                0.
            }
    }
    if let Ok(mut velocity) = query.get_single_mut() {
        velocity.y =
            if kb.pressed(KeyCode::Up) {
                1.
            } else if kb.pressed(KeyCode::Down) {
                -1.
            } else {
                0.
            }
    }
}


//// LAZY GIT reminder
//// Close the entire line? not just parts of the line with brackets?
