use bevy::{
    ecs::{
        archetype::Archetypes,
        component::{ComponentId, Components},
        entity::Entities,
        schedule::ShouldRun,
    },
    prelude::*,
    reflect::TypeRegistration,
};
use std::io::{self, BufRead};

struct Pause(bool);

fn pause(pause: Res<Pause>) -> ShouldRun {
    if pause.0 {
        ShouldRun::YesAndCheckAgain
    } else {
        ShouldRun::No
    }
}

fn parse_input(
    mut pause: ResMut<Pause>,
    archetypes: &Archetypes,
    components: &Components,
    entities: &Entities,
) {
    println!(">>>");
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut split = line.split_whitespace().into_iter();

    if let Some(first_arg) = split.next() {
        match first_arg {
            "exit" => pause.0 = false,
            "resources" => print_resources(archetypes, components),
            "components" => print_components(components),
            "counts" => print_ecs_counts(archetypes, components, entities),
            _ => println!("Command not supported"),
        }
    }
}

fn input_pause(keyboard_input: Res<Input<KeyCode>>, mut pause: ResMut<Pause>) {
    if keyboard_input.pressed(KeyCode::F10) {
        pause.0 = true;
    }
}

fn print_resources(archetypes: &Archetypes, components: &Components) {
    let mut r: Vec<String> = archetypes
        .resource()
        .components()
        .map(|id| components.get_info(id).unwrap())
        // get_short_name removes the path information
        // i.e. `bevy_audio::audio::Audio` -> `Audio`
        // if you want to see the path info replace
        // `TypeRegistration::get_short_name` with `String::from`
        .map(|info| TypeRegistration::get_short_name(info.name()))
        .collect();

    // sort list alphebetically
    r.sort();
    r.iter().for_each(|name| println!("{}", name));
}

fn print_components(components: &Components) {
    let mut names = Vec::new();
    for id in 1..components.len() {
        if let Some(info) = components.get_info(ComponentId::new(id)) {
            names.push((id, TypeRegistration::get_short_name(info.name())));
        }
    }

    // sort list alphebetically
    names.sort();
    names.iter().for_each(|(id, name)| println!("{} {}", id, name));
}

fn print_ecs_counts(a: &Archetypes, c: &Components, e: &Entities) {
    println!(
        "entities {}, components: {}, archetypes {}",
        e.len(),
        c.len(),
        a.len()
    );
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .add_system(input_pause.system())
            .add_system(parse_input.system().with_run_criteria(pause.system()));
    }
}
