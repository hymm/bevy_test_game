use crate::consts::AppState;
use crate::map::Levels;
use bevy::prelude::*;

struct RootNode;
struct VictoryScreen;
fn spawn_end_screen(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let handle = asset_server.load("sprites/victory_screen.png");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(handle.into()),
            transform: Transform {
                translation: Vec3::new(64.0, 64.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(VictoryScreen);

    commands
        .spawn()
        .insert_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent.spawn().insert_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "You Win!",
                    TextStyle {
                        font: asset_server.load("fonts/silkscreen/slkscreb.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(95.0, 87.0, 79.0),
                    },
                    Default::default(),
                ),
                ..Default::default()
            });

            parent.spawn().insert_bundle(TextBundle {
                style: Style {
                    margin: Rect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::with_section(
                    "Press X to Restart",
                    TextStyle {
                        font: asset_server.load("fonts/silkscreen/slkscreb.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(95.0, 87.0, 79.0),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Center,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            });
        });
}

fn restart(
    mut state: ResMut<State<AppState>>,
    mut levels: ResMut<Levels>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::X) {
        levels.current_level = 0;
        state.set(AppState::Loading).unwrap();
    }
}

fn despawn_win_screen(
    mut commands: Commands,
    text_query: Query<Entity, With<RootNode>>,
    background_query: Query<Entity, With<VictoryScreen>>,
) {
    if let Ok(entity) = text_query.single() {
        commands.entity(entity).despawn_recursive();
    }

    if let Ok(entity) = background_query.single() {
        commands.entity(entity).despawn();
    }
}

pub struct WinScreenPlugin;
impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Finished).with_system(spawn_end_screen.system()),
        )
        .add_system_set(SystemSet::on_update(AppState::Finished).with_system(restart.system()))
        .add_system_set(
            SystemSet::on_exit(AppState::Finished).with_system(despawn_win_screen.system()),
        );
    }
}
