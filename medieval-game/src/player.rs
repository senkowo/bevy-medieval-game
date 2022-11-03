use crate::{GameTextures, WinSize, PLAYER_SIZE, SPRITE_SCALE, TIME_STEP, BASE_SPEED, Velocity, Player};
use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
        // call in every event loop
        .add_system(player_movement_system);
    }
}

fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    win_size: Res<WinSize>, // custom Resource, so can access window size from startup system.
) {
    // --- Create sprite entity --- using SpriteBundle (a sprite builder)
    let bottom = -win_size.h / 2.;
    commands.spawn_bundle(
        SpriteBundle {
            // vvv Within the struct "SpriteBundle", there is a field called "texture", which we'll set
            //     to the handle/pointer/reference to the sprite image, returned from asset_server.load().
            texture: game_textures.player.clone(), // <- Added a New component (property) of sprite (entity).
            // vvv Another component (transforms sprite):
            transform: Transform {
                translation: Vec3::new(
                    0.,
                    (bottom + PLAYER_SIZE.1 / 2. * SPRITE_SCALE + 5.),
                    10.
                    //^ Default spawn location
                    //  Vec3::new(x, y, z),
                    //  Note: z-axis is the spite layer (layer here is set to 10. for now)
                ),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.),
                // ^^^ Scales the sprite
                ..Default::default()
            },
            ..Default::default() // use default properties for the rest of the sprite
        }
    )
    .insert(Player)
    .insert(Velocity {x: 0., y: 0.});
}

// keyboard inputs -> Velocity
fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query: Query<&mut Velocity>, With<Player>>

) {

}

fn player_movement_system(mut query: Query<(&Velocity, &mut Transform), With<Player>>) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;
    }
}

//// LAZY GIT reminder
