use bevy::prelude::*;
use rand::{thread_rng, Rng};
use crate::game::GameplayObject;

#[derive(Resource)]
pub struct EnemyShootTimer(pub Timer);

#[derive(Component)]
pub struct EnemiesStructInLevel1;

#[derive(Component)]
pub struct EnemiesStructInLevel2;

#[derive(Component)]
pub struct EnemiesStructInLevel3;

#[derive(Resource, Default)]
pub struct LevelEnemiesSpawned(pub bool);

#[derive(Default, States, Eq, PartialEq, Clone, Copy, Debug, Hash)]
pub enum LevelState {
    #[default]
    Level1,
    Level2,
    Level3,
}

pub fn load_level_1(mut commands: Commands, asset_server: Res<AssetServer>,
mut spawned: ResMut<LevelEnemiesSpawned>) {
    let image = asset_server.load("images/players/enemyShip.png");
    for _ in 0..7 {
        let x = thread_rng().gen_range(-380.0..380.0_f32);
        let y = thread_rng().gen_range(100.0..250.0_f32);
        commands.spawn((
            Sprite::from_image(image.clone()),
            Transform::from_xyz(x, y, 0.5),
            EnemiesStructInLevel1,
            GameplayObject
        ));
    }
    spawned.0 = true;
}

pub fn unload_level_1(
    mut commands: Commands,
    enemies: Query<Entity, With<EnemiesStructInLevel1>>,
    mut spawned: ResMut<LevelEnemiesSpawned>,
) {
    for e in enemies.iter() {
        commands.entity(e).despawn();
    }
    spawned.0 = false;
}

pub fn load_level_2(mut commands: Commands, asset_server: Res<AssetServer>,
                    mut spawned: ResMut<LevelEnemiesSpawned>) {
    let image = asset_server.load("images/players/enemyShip.png");
    for _ in 0..15 {
        let x = thread_rng().gen_range(-380.0..380.0_f32);
        let y = thread_rng().gen_range(50.0..250.0_f32);
        commands.spawn((
            Sprite::from_image(image.clone()),
            Transform::from_xyz(x, y, 0.5),
            EnemiesStructInLevel2,
            GameplayObject
        ));
    }
    spawned.0 = true;
}

pub fn unload_level_2(
    mut commands: Commands,
    enemies: Query<Entity, With<EnemiesStructInLevel2>>,
    mut spawned: ResMut<LevelEnemiesSpawned>
) {
    for e in enemies.iter() {
        commands.entity(e).despawn();
    }
    spawned.0 = false;
}

pub fn load_level_3(mut commands: Commands, asset_server: Res<AssetServer>,
                    mut spawned: ResMut<LevelEnemiesSpawned>) {
    let image = asset_server.load("images/players/enemyShip.png");
    for _ in 0..25 {
        let x = thread_rng().gen_range(-380.0..380.0_f32);
        let y = thread_rng().gen_range(0.0..250.0_f32);
        commands.spawn((
            Sprite::from_image(image.clone()),
            Transform::from_xyz(x, y, 0.5),
            EnemiesStructInLevel3,
            GameplayObject
        ));
    }
    spawned.0 = true;
}

pub fn unload_level_3(
    mut commands: Commands,
    enemies: Query<Entity, With<EnemiesStructInLevel3>>,
    mut spawned: ResMut<LevelEnemiesSpawned>
) {
    for e in enemies.iter() {
        commands.entity(e).despawn();
    }
    spawned.0 = false;
}

pub fn check_level_1_complete(
    enemies: Query<&EnemiesStructInLevel1>,
    level_state: Res<State<LevelState>>,
    spawned: Res<LevelEnemiesSpawned>,
    mut next_level: ResMut<NextState<LevelState>>,
) {
    if spawned.0 && enemies.is_empty() && *level_state.get() == LevelState::Level1 {
        next_level.set(LevelState::Level2);
    }
}

pub fn check_level_2_complete(
    enemies: Query<&EnemiesStructInLevel2>,
    level_state: Res<State<LevelState>>,
    spawned: Res<LevelEnemiesSpawned>,
    mut next_level: ResMut<NextState<LevelState>>,
) {
    if spawned.0 && enemies.is_empty() && *level_state.get() == LevelState::Level2 {
        next_level.set(LevelState::Level3);
    }
}

pub fn distance_between_enemies_in_level_1(
    mut enemies_q: Query<(Entity, &mut Transform), With<EnemiesStructInLevel1>>,
) {
    let minimum_distance = 100.0;
    let enemies: Vec<(Entity, Vec3)> = enemies_q.iter().map(|(e, t)| (e, t.translation)).collect();
    for (entity_1, mut transform_1) in enemies_q.iter_mut() {
        for (entity_2, pos2) in &enemies {
            if entity_1 != *entity_2 {
                let d = transform_1.translation - *pos2;
                let dist = d.length();
                if dist > 0.0 && dist < minimum_distance {
                    transform_1.translation += d.normalize() * (minimum_distance - dist);
                }
            }
        }
    }
}

pub fn distance_between_enemies_in_level_2(
    mut enemies_q: Query<(Entity, &mut Transform), With<EnemiesStructInLevel2>>,
) {
    let minimum_distance = 80.0;
    let enemies: Vec<(Entity, Vec3)> = enemies_q.iter().map(|(e, t)| (e, t.translation)).collect();
    for (entity_1, mut transform_1) in enemies_q.iter_mut() {
        for (entity_2, pos2) in &enemies {
            if entity_1 != *entity_2 {
                let d = transform_1.translation - *pos2;
                let dist = d.length();
                if dist > 0.0 && dist < minimum_distance {
                    transform_1.translation += d.normalize() * (minimum_distance - dist);
                }
            }
        }
    }
}

pub fn distance_between_enemies_in_level_3(
    mut enemies_q: Query<(Entity, &mut Transform), With<EnemiesStructInLevel3>>,
) {
    let minimum_distance = 60.0;
    let enemies: Vec<(Entity, Vec3)> = enemies_q.iter().map(|(e, t)| (e, t.translation)).collect();
    for (entity_1, mut transform_1) in enemies_q.iter_mut() {
        for (entity_2, pos2) in &enemies {
            if entity_1 != *entity_2 {
                let d = transform_1.translation - *pos2;
                let dist = d.length();
                if dist > 0.0 && dist < minimum_distance {
                    transform_1.translation += d.normalize() * (minimum_distance - dist);
                }
            }
        }
    }
}