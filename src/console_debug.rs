use bevy::{ecs::schedule::ShouldRun, prelude::*};
use std::io::{self, BufRead};

struct Pause(bool);

fn pause(pause: Res<Pause>) -> ShouldRun {
    if pause.0 {
        ShouldRun::YesAndCheckAgain
    } else {
        ShouldRun::No
    }
}

fn parse_input(mut pause: ResMut<Pause>) {
    println!("Entered Debug Console:");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    if line == "exit" {
        pause.0 = false
    }
}

fn input_pause(keyboard_input: Res<Input<KeyCode>>, mut pause: ResMut<Pause>) {
    if keyboard_input.pressed(KeyCode::J) {
        pause.0 = true;
    }
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .add_system(input_pause.system())
            .add_system(parse_input.system().with_run_criteria(pause.system()));
    }
}
