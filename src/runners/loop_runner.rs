use bevy::prelude::*;
use std::{io, io::BufRead};
use super::super::Input;
pub fn defalut(mut app: App) {
    for line in io::stdin().lock().lines() {
        {
            let mut input = app.world.resource_mut::<Input>();
            input.0 = line.unwrap();
        }
        app.update();
    }
}
