use bevy::{
    ecs::{
        archetype::Archetypes,
        component::{ComponentId, Components},
        entity::{Entities, Entity},
        schedule::ShouldRun,
    },
    prelude::*,
    reflect::TypeRegistration,
};
use clap::{App, Arg, ArgGroup, ArgSettings};
use std::io::{self, BufRead};

struct Pause(bool);

fn parse_input(mut pause: ResMut<Pause>, a: &Archetypes, c: &Components, e: &Entities) {
    println!(">>>");
    let app_name = "BevyConsoleDebugger";
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let split = line.split_whitespace();
    let mut args = vec![app_name];
    args.append(&mut split.collect());

    let matches_result = App::new(app_name)
        .subcommand(App::new("exit").about("exit debug mode"))
        .subcommand(
            App::new("counts").about("print counts of archetypes, components, and entities"),
        )
        .subcommand(
            App::new("list")
                .about("list components, archetypes, entities")
                .arg("--resources 'List resources'")
                .arg("--components 'List compoenets'")
                .group(ArgGroup::new("types").args(&["resources", "components"])),
        )
        .subcommand(
            App::new("find")
                .about("Find archetypes and enties")
                .arg("--archetype   'Returns archetype ids matching filters'")
                .arg("--entity      'Returns entity ids matching filters'")
                .group(ArgGroup::new("search types").args(&["archetype", "entity"]))
                .arg("--componentid=[ComponentId] 'Find by component id'"
                )
                .arg("--entityid    'Find by --entityid=<EntityId>, only works for --archetype"),
        )
        .try_get_matches_from(args);

    if let Err(e) = matches_result {
        println!("{}", e.to_string());
        return;
    }

    let matches = matches_result.unwrap();

    if let Some(_) = matches.subcommand_matches("exit") {
        pause.0 = false;
    }

    if let Some(matches) = matches.subcommand_matches("list") {
        if matches.is_present("resources") {
            print_resources(a, c);
        }

        if matches.is_present("components") {
            print_components(c);
        }
    }

    if let Some(_) = matches.subcommand_matches("counts") {
        print_ecs_counts(a, c, e);
    }

    if let Some(matches) = matches.subcommand_matches("find") {
        if matches.is_present("archetype") {
            let component_id = matches.value_of_t("componentid").unwrap();
            find_archetypes(a, Some(component_id), None);
        }
    }
}

fn pause(pause: Res<Pause>) -> ShouldRun {
    if pause.0 {
        ShouldRun::YesAndCheckAgain
    } else {
        ShouldRun::No
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
    names
        .iter()
        .for_each(|(id, name)| println!("{} {}", id, name));
}

fn print_ecs_counts(a: &Archetypes, c: &Components, e: &Entities) {
    println!(
        "entities {}, components: {}, archetypes {}",
        e.len(),
        c.len(),
        a.len()
    );
}

fn find_archetypes(a: &Archetypes, component_id: Option<usize>, entity_id: Option<u32>) {
    if let Some(component_id) = component_id {
        let archetypes = a
            .iter()
            .filter(|archetype| archetype.components().any(|c| c.index() == component_id))
            .map(|archetype| archetype.id().index());
        archetypes.for_each(|id| println!("{}", id));
    };
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .add_system(input_pause.system())
            .add_system(parse_input.system().with_run_criteria(pause.system()));
    }
}
