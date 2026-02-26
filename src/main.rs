mod camera;
mod game;
mod player;
mod lasers_player;
mod lasers_enemies;
mod levels;

use bevy::prelude::*;
use bevy::window::*;

use camera::*;
use crate::levels::*;
use crate::game::*;
use crate::lasers_enemies::*;
use crate::lasers_player::*;
use crate::player::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space Shooter in Rust with Bevy".to_string(),
                resolution: WindowResolution::new(1000, 801).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        })
    )
        .init_state::<GameState>()
        .init_state::<LevelState>()
        .insert_resource(EnemyShootTimer(Timer::from_seconds(3.5, TimerMode::Repeating)))
        .insert_resource(LevelEnemiesSpawned(false))
        .add_systems(Startup, (camera_setup, load_background_for_game, load_players))
        .add_systems(OnEnter(GameState::NotStarted), show_start_text)
        .add_systems(OnExit(GameState::NotStarted), clean_start_text)
        .add_systems(Update, start.run_if(in_state(GameState::NotStarted)))
        .add_systems(OnEnter(LevelState::Level1), load_level_1)
        .add_systems(OnExit(LevelState::Level1), unload_level_1)
        .add_systems(OnEnter(LevelState::Level2), load_level_2)
        .add_systems(OnExit(LevelState::Level2), unload_level_2)
        .add_systems(OnEnter(LevelState::Level3), load_level_3)
        .add_systems(OnExit(LevelState::Level3), unload_level_3)
        .add_systems(Update, (keys_for_players, borders_for_player)
            .run_if(in_state(GameState::InGame)))
        .add_systems(Update, (lasers_player, move_lasers, collision_lasers_player_with_enemies)
            .run_if(in_state(GameState::InGame)))
        .add_systems(Update, (enemies_shoot, move_lasers_enemies, collision_lasers_player_with_lasers_enemies)
            .run_if(in_state(GameState::InGame)))
        .add_systems(Update, distance_between_enemies_in_level_1
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level1))))
        .add_systems(Update, distance_between_enemies_in_level_2
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level2))))
        .add_systems(Update, distance_between_enemies_in_level_3
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level3))))
        .add_systems(Update, check_level_1_complete
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level1))))
        .add_systems(Update, check_level_2_complete
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level2))))
        .add_systems(Update, keys)
        .add_systems(Update, game_over.run_if(in_state(GameState::InGame)))
        .add_systems(Update, win_game
            .run_if(in_state(GameState::InGame).and(in_state(LevelState::Level3))))
        .add_systems(Update, restart_game
            .run_if(in_state(GameState::GameOver).or(in_state(GameState::Win))))
        .add_systems(Update, update_gameplay)
        .run();
}
 