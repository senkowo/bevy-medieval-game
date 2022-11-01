//
//
// Bevy Medieval Game thing -- Kai, Renzo, Caleb
// uwu
//

//# Special Settings: #//
#![allow(unused)] // silence warnings given by the compiler saying that a variable is not in use. Delete this line later.


//---------------------------------------------------------
//### General Notes: ###
// Bevy ECS types: >> System <|> Components <|> Entity <<
//
//---------------------------------------------------------


//_ IMPORTS _//
use bevy::prelude::*; // import basic stuff



//---------------------------------------------------------
//# CONSTANTS #//

//# Asset Constants #
// region:
const PLAYER_SPRITE: &str = "horse1.png";
const PLAYER_SIZE: (f32, f32) = (144., 75.);
// endregion:

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
  // Add default plugins to make the program run correctly.
  .add_plugins(DefaultPlugins)
  // --- Add Startup systems below ---
  .add_startup_system(setup_system)
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

  // -- Main sprite entity -- using SpriteBundle (sprite builder)
  let bottom = -win_h / 2.;
  commands.spawn_bundle(SpriteBundle {
    // Within the struct "SpriteBundle", there is a field called "texture", which we'll set
    //  to the handle/pointer/reference to the sprite image, returned from asset_server.load().
    texture: asset_server.load(PLAYER_SPRITE), // <- New component (property) of sprite (entity).
    // Another component (transforms sprite):
    transform: Transform {
      translation: Vec3::new(0., bottom + PLAYER_SIZE.1 / 2. + 5., 10.),
      ..Default::default()
    },
    ..Default::default() // use default properties for the rest of the sprite
  });
}
