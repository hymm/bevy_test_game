use crate::consts::AppState;
use crate::map::Levels;
use bevy::prelude::*;

#[derive(Component)]
struct RootNode;

#[derive(Component)]
struct VictoryScreen;
fn spawn_end_screen(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle = asset_server.load("sprites/victory_screen.png");
    commands
        .spawn(SpriteBundle {
            texture: handle,
            transform: Transform {
                translation: Vec3::new(64.0, 64.0, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(VictoryScreen);

    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                flex_direction: FlexDirection::ColumnReverse,
                justify_content: JustifyContent::Center,
                align_content: AlignContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            background_color: Color::rgba(0.0, 0.0, 0.0, 0.0).into(),
            ..Default::default()
        })
        .insert(RootNode)
        .with_children(|parent| {
            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "You Win!",
                    TextStyle {
                        font: asset_server.load("fonts/silkscreen/slkscreb.ttf"),
                        font_size: 40.0,
                        color: Color::rgb(0.37, 0.34, 0.31),
                    },
                ),
                ..Default::default()
            });

            parent.spawn(TextBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(5.0)),
                    ..Default::default()
                },
                text: Text::from_section(
                    "Press X to Restart",
                    TextStyle {
                        font: asset_server.load("fonts/silkscreen/slkscreb.ttf"),
                        font_size: 16.0,
                        color: Color::rgb(0.37, 0.34, 0.31),
                    },
                )
                .with_alignment(TextAlignment::Center),
                ..Default::default()
            });
        });
}

fn restart(
    mut state: ResMut<NextState<AppState>>,
    mut levels: ResMut<Levels>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::X) {
        levels.current_level = 0;
        state.set(AppState::Loading);
    }
}

fn despawn_win_screen(
    mut commands: Commands,
    text_query: Query<Entity, With<RootNode>>,
    background_query: Query<Entity, With<VictoryScreen>>,
) {
    let entity = text_query.single();
    commands.entity(entity).despawn_recursive();

    let entity = background_query.single();
    commands.entity(entity).despawn();
}

pub struct WinScreenPlugin;
impl Plugin for WinScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_end_screen.in_schedule(OnEnter(AppState::Finished)))
            .add_system(restart.in_set(OnUpdate(AppState::Finished)))
            .add_system(despawn_win_screen.in_schedule(OnExit(AppState::Finished)));
    }
}
