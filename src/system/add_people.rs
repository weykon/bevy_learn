use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;

pub fn add_people(mut commands: Commands){
    commands.spawn().insert(Person).insert(Name("Elaina Proctor".to_string()));
    commands.spawn().insert(Person).insert(Name("Renzo Hume".to_string()));
    commands.spawn().insert(Person).insert(Name("Zayna Nieves".to_string()));
}