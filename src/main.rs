use bevy::core::*;
use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;
mod runners;
mod system;
pub struct Input(String);

fn main() {
    println!("Hello, world!");
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(Input(String::new()))
        .add_startup_system(system::add_people::add_people)
        .set_runner(runners::loop_runner::defalut)
        .add_system(system::print_system::defalut)
        .add_system(system::hello_system::defalut)
        .run();
    // .add_startup_system(system::setup_3d_scene::setup)
}
