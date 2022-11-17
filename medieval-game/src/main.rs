//
//
// Bevy Medieval Game thing -- Kai, Renzo, Caleb
// uwu
//
//

//# Special Settings: #//
#![allow(unused)] // silence warnings given by the compiler saying that a variable is not in use. Delete this line later.


//---------------------------------------------------------
//### General Notes: ###
// Bevy ECS types: System -> Components -> Entity
//
//---------------------------------------------------------


//_ IMPORTS _//
use bevy::prelude::*; // import basic stuff
use crate::player::PlayerPlugin; // <- PlayerPlugin implements the Plugin trait and has a build
                                 //    method function containing all of its systems, and they all
                                 //    run below in App::new() <- add_plugin(PlayerPlugin).
use crate::enemy::EnemyPlugin;
use crate::components::*;

//_ "PLUGINS" _// (not sure what this does)
mod components; // <- general bevy ECS components
mod enemy;
mod player;



//---------------------------------------------------------
//#--- Asset Constants ---#

const PLAYER_SPRITE: &str = "horse1.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
const PLAYER_SCALE: f32 = 1.;
const PLAYER_ARROW_SPRITE: &str = "arrow1.png";
const ARROW_SIZE: (f32, f32) = (9., 54.);
const ARROW_SCALE: f32 = 0.05;

const ENEMY_SPRITE: &str = "horse1.png";
const ENEMY_SIZE: (f32, f32) = (144., 75.);
const ENEMY_PROJ_SPRITE: &str = "enemy_proj1";
const ENEMY_PROJ_SIZE: (f32, f32) = (17., 55.);


//#--- Game Constants ---#

const TIME_STEP: f32 = 1. / 60.; // <- 60 fps
const BASE_SPEED: f32 = 500.;


//#--- Resources (custom) ---#

// access window size from different systems
pub struct WinSize {
    pub w: f32,
    pub h: f32,
}

// access to asset_server without lots of imports (simply access the struct)
// This is a struct, so it doesn't store anything by default. It's more like a framework or Class.
// Basically, I can now import this struct with "use crate::GameTextures" in other files.
pub struct GameTextures {
    player: Handle<Image>,
    player_arrow: Handle<Image>,
    enemy: Handle<Image>,
    enemy_laser: Handle<Image>,
}





//---------------------------------------------------------



//---------------------------------------------
//##_ BEGIN CODE _##//
//

fn main() {
    App::new()
        // Default background color
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        // Customizing window.
        .insert_resource(WindowDescriptor {
            title: "Rust Invaders!".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        // Add plugins --
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        // Add setup_system as the 'startup' system
        .add_startup_system(setup_system)
        // Add movable_system to keep it continuously running
        .add_system(movable_system)
        // Run the program
        .run();
}

// This is the system (or function) that sets up the game.
// The "commands" variable of type 'Commands' (from bevy::prelude::Commands) allows you
//   to add or remove things from the game scene.
// The "asset_server" of type 'Res<AssetServer>' is a resource (Res<_>) that loads resources
//   from the filesystem (e.g. images for sprites) and injects them as an argument as
//   appropriate (it's a Generic variable, so it'll adapt to load any file type).
// The "windows" variable of type 'ResMut<Windows>' is a mutable resource (ResMut<_>) that
//   will allow you to access and alter the window's components (properties).
fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut windows: ResMut<Windows>,
) {
    commands.spawn_bundle(Camera2dBundle::default()); // spawn camera entity
    //commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // --- Capture window size
    // Get window properties from the "windows" resource of type 'ResMut<Windows>'.
    let window = windows.get_primary_mut().unwrap();
    let (win_w, win_h) = (window.width(), window.height());

    // position window (for tutorial)
    window.set_position(IVec2::new(2780, 4900));

    // add WinSize resource
    let win_size = WinSize {w: win_w, h: win_h};
    commands.insert_resource(win_size);

    // Creates an instance of GameTextures
    // Its instance variables are bound to the sprite files (e.g. PLAYER_ARROW_SPRITE).
    // So, to access the sprite files from another file, simply import this struct with
    //   "use crate::GameTextures".
    // Then, access the sprite file bound to the variable, within this struct instance
    //   (game_textures): "game_textures.<instance-variable>".
    let game_textures = GameTextures {
        player: asset_server.load(PLAYER_SPRITE),
        player_arrow: asset_server.load(PLAYER_ARROW_SPRITE),
        enemy: asset_server.load(ENEMY_SPRITE),
        enemy_laser: asset_server.load(ENEMY_PROJ_SPRITE),
    };
    commands.insert_resource(game_textures);

    /*
     * Note: Player spawning mechanism moved into a separate system.
    */
}

// movement of other sprites/entities
fn movable_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    mut query: Query<(Entity, &Velocity, &mut Transform, &Movable)>) {
    for (entity, velocity, mut transform, movable) in query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED;
        translation.y += velocity.y * TIME_STEP * BASE_SPEED;


        if movable.auto_despawn {
            // despawn when out of screen
            const MARGIN: f32 = 200.;
            if translation.y > win_size.h / 2. + MARGIN
                || translation.y < -win_size.h / 2. - MARGIN
                || translation.x > win_size.h / 2. + MARGIN
                || translation.x < -win_size.h / 2. - MARGIN
            {
                commands.entity(entity).despawn();
            }
        }
    }
}
