use bevy::{
    ecs::{
        archetype::{ArchetypeId, Archetypes},
        component::{ComponentId, Components, StorageType},
        entity::Entities,
        schedule::ShouldRun,
    },
    prelude::*,
    reflect::TypeRegistration,
};
use clap::{App, Arg, ArgGroup, ArgSettings};
use std::io::{self, BufRead, Write};
use std::process::exit;
#[derive(Default)]
struct Pause(bool);
fn parse_input(world: &mut World) {
    let a = world.archetypes();
    let c = world.components();
    let e = world.entities();
    let entering_console = world.get_resource::<EnteringConsole>().unwrap();
    if entering_console.0 {
        println!("Bevy Console Debugger.  Type 'help' for list of commands.");
    }
    print!(">>> ");
    io::stdout().flush().unwrap();
    let app_name = "";
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();

    println!("");
    let split = line.split_whitespace();
    let mut args = vec![app_name];
    args.append(&mut split.collect());

    let matches_result = App::new(app_name)
        .subcommand(App::new("resume").about("resume running game"))
        .subcommand(App::new("quit").about("quit game"))
        .subcommand(
            App::new("counts").about("print counts of archetypes, components, and entities"),
        )
        .subcommand(
            App::new("list")
                .about("print a list of <type>")
                .arg(
                    Arg::new("type")
                        .index(1)
                        .possible_values(&[
                            "archetypes",
                            "components", 
                            "entities",
                            "systems",
                            "resources",
                        ])
                        .required(true)
                ).arg("--filter [Filter] 'filter list'"),
        )
        .subcommand(
            App::new("find")
                .about("find archetypes, systems, and entities")
                .arg(
                    Arg::new("type")
                        .index(1)
                        .possible_values(&[
                            "archetypes", "archetype", 
                            "entities", "entity", 
                            "systems", "system"
                        ]))
                .arg("--componentid   [ComponentId]   'find types that have components with ComponentId'")
                .arg("--componentname [ComponentName] 'find types that have components with ComponentName'")
                .arg("--entityid      [EntityId]      'find types that have entities with EntityId, only works for archetypes"),
        )
        .subcommand(
            App::new("info")
                .about("get info about a single thing")
                .arg(Arg::new("type").index(1).possible_values(&[
                    "archetype",
                    "component", 
                    "entity", 
                    "system",
                ]))
                .arg("--id   [Id]   'id to get'")
                .arg("--name [Name] 'name to get, only works for component and system'"),
        )
        .try_get_matches_from(args);

    if let Err(e) = matches_result {
        println!("{}", e.to_string());
        return;
    }

    let matches = matches_result.unwrap();

    match matches.subcommand() {
        Some(("resume", _)) => {
            let mut pause = world.get_resource_mut::<Pause>().unwrap();
            pause.0 = false;
            println!("...resuming game.")
        }
        Some(("quit", _)) => exit(0),
        Some(("list", matches)) => match matches.value_of("type") {
            Some(t) => match t {
                "archetypes" | "archetype" => list_archetypes(a),
                "entities" | "entity" => list_entities(e),
                "resources" | "resource" => list_resources(a, c),
                "components" | "component" => list_components(c),
                "systems" | "system" => {}
                _ => {}
            },
            None => {}
        },
        Some(("counts", _)) => print_ecs_counts(a, c, e),
        Some(("find", matches)) => match matches.value_of("type") {
            Some(t) => match t {
                "archetypes" | "archetype" => {
                    if let Ok(component_id) = matches.value_of_t("componentid") {
                        find_archetypes_by_component_id(a, component_id);
                    }

                    if let Ok(entity_id) = matches.value_of_t("entityid") {
                        find_archetypes_by_entity_id(a, entity_id);
                    }
                }
                "entities" | "entity" => {
                    if let Ok(component_id) = matches.value_of_t("componentid") {
                        find_entities_by_component_id(a, e, component_id);
                    }
                }
                _ => {}
            },
            None => {}
        },
        Some(("info", matches)) => match matches.value_of("type") {
            Some(t) => match t {
                "archetype" => {
                    if let Ok(id) = matches.value_of_t("id") {
                        print_archetype(a, c, ArchetypeId::new(id));
                    }
                }
                "component" => {
                    if let Ok(id) = matches.value_of_t("id") {
                        print_component(c, id);
                    }
                }
                _ => {}
            },
            None => println!("invalid type: archetype"),
        },
        _ => {}
    }

    println!("");
}

struct EnteringConsole(bool);
fn pause(
    pause: Res<Pause>,
    mut last_pause: Local<Pause>,
    mut entering_console: ResMut<EnteringConsole>,
) -> ShouldRun {
    entering_console.0 = (pause.0 != last_pause.0) && pause.0;
    last_pause.0 = pause.0;
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

fn list_resources(archetypes: &Archetypes, components: &Components) {
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

fn list_components(components: &Components) {
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

fn list_entities(e: &Entities) {
    println!("[entity index] [archetype id]");
    e.meta.iter().enumerate().for_each(|(id, meta)| {
        println!("{} {}", id, meta.location.archetype_id.index());
    });
}

fn list_archetypes(a: &Archetypes) {
    println!("[id] [entity count]");
    a.iter().for_each(|archetype| {
        println!(
            "{} {}",
            archetype.id().index(),
            archetype.entities().iter().count()
        )
    });
}

fn print_ecs_counts(a: &Archetypes, c: &Components, e: &Entities) {
    println!(
        "entities {}, components: {}, archetypes {}",
        e.len(),
        c.len(),
        a.len()
    );
}

fn find_archetypes_by_component_id(a: &Archetypes, component_id: usize) {
    let archetypes = a
        .iter()
        .filter(|archetype| archetype.components().any(|c| c.index() == component_id))
        .map(|archetype| archetype.id().index());
    archetypes.for_each(|id| println!("{}", id));
}

fn find_archetypes_by_entity_id(a: &Archetypes, entity_id: u32) {
    let archetypes = a
        .iter()
        .filter(|archetype| archetype.entities().iter().any(|e| e.id() == entity_id))
        .map(|archetype| archetype.id().index());
    archetypes.for_each(|id| println!("{}", id));
}

fn find_entities_by_component_id(a: &Archetypes, e: &Entities, component_id: usize) {
    let entities = a
        .iter()
        .filter(|archetype| archetype.components().any(|c| c.index() == component_id))
        .map(|archetype| archetype.entities())
        .flatten();

    entities.for_each(|id| println!("{}", id.id()));
}

fn print_archetype(a: &Archetypes, c: &Components, archetype_id: ArchetypeId) {
    if let Some(archetype) = a.get(archetype_id) {
        println!("id: {:?}", archetype.id());
        println!("table_id: {:?}", archetype.table_id());
        print!("entities ({}): ", archetype.entities().iter().count());
        archetype
            .entities()
            .iter()
            .for_each(|entity| print!("{}, ", entity.id()));
        println!("");
        // not sure what entity table rows is, so commenting out for now
        // print!(
        //     "entity table rows ({}): ",
        //     archetype.entity_table_rows().iter().count()
        // );
        // archetype
        //     .entity_table_rows()
        //     .iter()
        //     .for_each(|row| print!("{}, ", row));
        // println!("");
        print!(
            "table_components ({}): ",
            archetype.table_components().iter().count()
        );
        archetype
            .table_components()
            .iter()
            .map(|id| (id.index(), c.get_info(*id).unwrap()))
            .map(|(id, info)| (id, TypeRegistration::get_short_name(info.name())))
            .for_each(|(id, name)| print!("{} {}, ", id, name));
        println!("");

        print!(
            "sparse set components ({}): ",
            archetype.sparse_set_components().iter().count()
        );
        archetype
            .sparse_set_components()
            .iter()
            .map(|id| (id.index(), c.get_info(*id).unwrap()))
            .map(|(id, info)| (id, TypeRegistration::get_short_name(info.name())))
            .for_each(|(id, name)| print!("{} {}, ", id, name));
        println!("");
    }
}

fn print_component(c: &Components, component_id: usize) {
    if let Some(info) = c.get_info(ComponentId::new(component_id)) {
        println!("Name: {}", info.name());
        println!("Id: {}", info.id().index());
        print!("StorageType: ");
        match info.storage_type() {
            StorageType::Table => println!("Table"),
            StorageType::SparseSet => println!("SparseSet"),
        }

        println!("SendAndSync: {}", info.is_send_and_sync());
    }
}

pub struct ConsoleDebugPlugin;
impl Plugin for ConsoleDebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(Pause(false))
            .insert_resource(EnteringConsole(false))
            .add_system(input_pause.system())
            .add_system(
                parse_input
                    .exclusive_system()
                    .with_run_criteria(pause.system()),
            );
    }
}
