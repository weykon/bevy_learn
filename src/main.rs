use bevy::core::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;
mod system;

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugin(CorePlugin::default())
        .add_plugin(InputPlugin::default())
        .add_plugin(WindowPlugin::default())
        .add_startup_system(system::add_people::add_people)
        .add_system(system::hello_system::hello_system)
        .run();
    // .add_startup_system(system::setup_3d_scene::setup)
}
