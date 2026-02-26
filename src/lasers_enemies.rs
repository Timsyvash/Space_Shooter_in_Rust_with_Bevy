use bevy::prelude::*;
use crate::game::GameplayObject;
use crate::levels::*;
use crate::lasers_player::LasersPlayerStruct;

#[derive(Component)]
pub struct LasersEnemiesStruct;

pub fn enemies_shoot(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut timer: ResMut<EnemyShootTimer>,
    mut enemies_l1_query: Query<&Transform, With<EnemiesStructInLevel1>>,
    mut enemies_l2_query: Query<&Transform, With<EnemiesStructInLevel2>>,
    mut enemies_l3_query: Query<&Transform, With<EnemiesStructInLevel3>>,
    time: Res<Time>
) {
    if timer.0.tick(time.delta()).just_finished() {
        let mut shoot = false;
        for enemy_transform in enemies_l1_query.iter_mut().chain(enemies_l2_query.iter_mut())
            .chain(enemies_l3_query.iter_mut()){
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/lasers/laserRed.png"),
                    ..default()
                },
                Transform::from_xyz(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                    0.5
                ),
                LasersEnemiesStruct,
                GameplayObject
            ));
            shoot = true;
        }
        if shoot {
            commands.spawn((
                AudioPlayer::new(asset_server.load("sounds/enemies_laser_music.ogg")),
                PlaybackSettings::ONCE
            ));
        }
    }
}

pub fn collision_lasers_player_with_lasers_enemies(
    lasers_player_query: Query<(&Transform, Entity), With<LasersPlayerStruct>>,
    lasers_enemies_query: Query<(&Transform, Entity), With<LasersEnemiesStruct>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (lasers_player_t, lasers_player_e) in lasers_player_query.iter() {
        for (lasers_enemies_t, lasers_enemies_e) in lasers_enemies_query.iter() {
            let collision = lasers_player_t.translation.y > lasers_enemies_t.translation.y - 10.0
            && lasers_player_t.translation.y < lasers_enemies_t.translation.y + 10.0
            && (lasers_player_t.translation.x - lasers_enemies_t.translation.x).abs() < 10.0;

            if collision {
                commands.entity(lasers_player_e).try_despawn();
                commands.entity(lasers_enemies_e).try_despawn();
                commands.spawn((
                    AudioPlayer::new(
                        asset_server.load("sounds/collision_of_the_player's_laser_with_the_enemy's_laser.ogg")
                    ),
                    PlaybackSettings::ONCE
                ));
            }
        }
    }
}

pub fn move_lasers_enemies(mut lasers_enemies_query:
                           Query<&mut Transform, With<LasersEnemiesStruct>>,
time: Res<Time>) {
    for mut t in lasers_enemies_query.iter_mut() {
        t.translation.y -= 365.0 * time.delta_secs();
    }
}
