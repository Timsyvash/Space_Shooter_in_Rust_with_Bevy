use bevy::prelude::*;
use crate::levels::*;
use crate::lasers_enemies::LasersEnemiesStruct;
use crate::lasers_player::LasersPlayerStruct;
use crate::player::PlayerStruct;

#[derive(Component)]
pub struct BackgroundStruct;

#[derive(Component)]
pub struct PauseStruct;

#[derive(Component)]
pub struct GameOverStruct;

#[derive(Component)]
pub struct WinStruct;

#[derive(Component)]
pub struct NotStartedStruct;

#[derive(Component)]
pub struct GameplayObject;

#[derive(Default, States, Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum GameState {
    #[default]
    NotStarted,
    InGame,
    GameOver,
    Pause,
    Win
}

pub fn update_gameplay(
    state: Res<State<GameState>>,
    mut query: Query<&mut Visibility, With<GameplayObject>>
) {
    let v = *state.get() == GameState::InGame;
    let vy = if v {Visibility::Visible} else { Visibility::Hidden };
    for mut v2 in query.iter_mut() {
        *v2 = vy
    }
}

pub fn load_background_for_game(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: asset_server.load("images/backgrounds/Background_for_game.png"),
            ..default()
        },
        Transform::from_xyz(0.0, 0.0, 0.0),
        BackgroundStruct
    ));
}

pub fn keys(
    mut next_state: ResMut<NextState<GameState>>,
    key_code: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut commands: Commands,
    pause_query: Query<Entity, With<PauseStruct>>,
    asset_server: Res<AssetServer>
) {
    if key_code.just_pressed(KeyCode::KeyP) {
        if *state.get() == GameState::InGame {
            next_state.set(GameState::Pause);
            commands.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                PauseStruct
            )).with_children(|parent| {
                parent.spawn((
                    Text::new("Пауза"),
                    TextFont {
                        font: asset_server.load("fonts/e-UkraineHead-Medium.otf"),
                        font_size: 60.0,
                        ..default()
                    },
                    TextColor(Color::WHITE),
                ));
            });

            commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/pause.ogg")),
                PlaybackSettings::ONCE,
            ));
        } else if *state.get() == GameState::Pause {
            next_state.set(GameState::InGame);
            for entity in pause_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn show_start_text(
    mut commands: Commands,
    state: Res<State<GameState>>,
    asset_server: Res<AssetServer>,
) {
    if *state.get() == GameState::NotStarted {
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            NotStartedStruct,
        )).with_children(|p| {
            p.spawn((
                Text::new("Гра не розпочата, натисніть на S для старту гри"),
                TextFont {
                    font: asset_server.load("fonts/e-ukrainehead-bold_w.ttf"),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
    }
}

pub fn clean_start_text(
    mut commands: Commands,
    not_started_query: Query<Entity, With<NotStartedStruct>>
) {
    for e in not_started_query.iter() {
        commands.entity(e).despawn();
    }
}

pub fn start(
    key_code: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>
) {
    if key_code.just_pressed(KeyCode::KeyS) && *state.get() == GameState::NotStarted {
        next_level.set(LevelState::Level1);
        next_state.set(GameState::InGame);
    }
}

pub fn game_over(
    mut commands: Commands,
    player_query: Query<(&Transform, Entity), With<PlayerStruct>>,
    lasers_enemies_query: Query<(&Transform, Entity), With<LasersEnemiesStruct>>,
    state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    player_lasers_query: Query<Entity, With<LasersPlayerStruct>>,
    enemies_query_l1: Query<Entity, With<EnemiesStructInLevel1>>,
    enemies_query_l2: Query<Entity, With<EnemiesStructInLevel2>>,
    enemies_query_l3: Query<Entity, With<EnemiesStructInLevel3>>,
) {
    for (player_tr, player_entity) in player_query.iter() {
        for (lasers_enemies_tr, lasers_enemies_entity) in lasers_enemies_query.iter() {
            let collision = lasers_enemies_tr.translation.y > player_tr.translation.y - 20.0
                && lasers_enemies_tr.translation.y < player_tr.translation.y + 20.0
                && (lasers_enemies_tr.translation.x - player_tr.translation.x).abs() < 20.0;

            if collision && *state.get() == GameState::InGame {
                next_state.set(GameState::GameOver);
                commands.spawn((
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        position_type: PositionType::Absolute,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    GameOverStruct,
                )).with_children(|parent| {
                    parent.spawn((
                        Text::new("Гра програна! Натисніть на R для рестарту гри"),
                        TextFont {
                            font: asset_server.load("fonts/e-Ukraine-Bold.otf"),
                            font_size: 25.0,
                            ..default()
                        },
                        TextColor(Color::WHITE)
                    ));
                });
                commands.spawn((
                    AudioPlayer::new(asset_server.load("sounds/game_over.ogg")),
                    PlaybackSettings::ONCE,
                ));
                commands.entity(player_entity).despawn();
                commands.entity(lasers_enemies_entity).despawn();
                for e in player_lasers_query.iter() {
                    commands.entity(e).despawn();
                }
                for e_l1 in enemies_query_l1.iter() {
                    for e_l2 in enemies_query_l2.iter() {
                        for e_l3 in enemies_query_l3.iter() {
                            commands.entity(e_l1).despawn();
                            commands.entity(e_l2).despawn();
                            commands.entity(e_l3).despawn();
                        }
                    }
                }
                return;
            }
        }
    }
}

pub fn restart_game(
    key_code: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_level: ResMut<NextState<LevelState>>,
    mut commands: Commands,
    game_over_query: Query<Entity, With<GameOverStruct>>,
    win_query: Query<Entity, With<WinStruct>>,
    enemies_l1: Query<Entity, With<EnemiesStructInLevel1>>,
    enemies_l2: Query<Entity, With<EnemiesStructInLevel2>>,
    enemies_l3: Query<Entity, With<EnemiesStructInLevel3>>,
    lasers_enemies_query: Query<Entity, With<LasersEnemiesStruct>>,
    lasers_player_query: Query<Entity, With<LasersPlayerStruct>>,
    asset_server: Res<AssetServer>,
    mut shoot_timer: ResMut<EnemyShootTimer>,
    mut spawned: ResMut<LevelEnemiesSpawned>,
    state: Res<State<GameState>>
) {
    if key_code.just_pressed(KeyCode::KeyR) {
        for e in game_over_query.iter() { commands.entity(e).despawn(); }
        for e in win_query.iter() { commands.entity(e).despawn(); }
        for e in enemies_l1.iter() { commands.entity(e).try_despawn(); }
        for e in enemies_l2.iter() { commands.entity(e).try_despawn(); }
        for e in enemies_l3.iter() { commands.entity(e).try_despawn(); }
        for e in lasers_enemies_query.iter() { commands.entity(e).try_despawn(); }
        for e in lasers_player_query.iter() { commands.entity(e).try_despawn(); }

        shoot_timer.0.reset();
        spawned.0 = false;

        if *state.get() == GameState::GameOver {
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/players/player.png"),
                    ..default()
                },
                Transform::from_xyz(0.0, -370.0, 0.5),
                PlayerStruct,
                GameplayObject
            ));
        }

        next_level.set(LevelState::Level1);
        next_state.set(GameState::NotStarted);
    }
}

pub fn win_game(
    mut commands: Commands,
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    enemies_l3: Query<&EnemiesStructInLevel3>,
    spawned: Res<LevelEnemiesSpawned>,
) {
    if spawned.0 && enemies_l3.is_empty() {
        next_state.set(GameState::Win);
        commands.spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            WinStruct,
        )).with_children(|p| {
            p.spawn((
                Text::new("Ви виграли гру! Натисніть R для рестарту"),
                TextFont {
                    font: asset_server.load("fonts/e-UkraineHead-Bold.otf"),
                    font_size: 25.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/win.ogg")),
            PlaybackSettings::ONCE,
        ));
    }
}
