use bevy::prelude::*;
use super::super::Input;


pub fn defalut(input: Res<Input>) {
    println!("input: {}", input.0);
}
