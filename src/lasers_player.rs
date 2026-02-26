use bevy::prelude::*;
use crate::game::GameplayObject;
use crate::levels::*;
use crate::player::*;

#[derive(Component)]
pub struct LasersPlayerStruct;

pub fn lasers_player(asset_server: Res<AssetServer>, mut commands: Commands,
mut player_query: Query<&Transform, With<PlayerStruct>>, key_code: Res<ButtonInput<KeyCode>>) {
    if key_code.just_pressed(KeyCode::Space) {
        for player_transform in player_query.iter_mut() {
            commands.spawn((
                Sprite {
                    image: asset_server.load("images/lasers/laserGreen.png"),
                    ..default()
                },
                Transform::from_xyz(player_transform.translation.x,
                                    player_transform.translation.y,
                                    0.5),
                LasersPlayerStruct,
                GameplayObject
            ));
        }
        commands.spawn((
            AudioPlayer::new(asset_server.load("sounds/player_laser_music.ogg")),
            PlaybackSettings::ONCE,
        ));
    }
}

pub fn move_lasers(
    time: Res<Time>,
    mut lasers_query: Query<&mut Transform, With<LasersPlayerStruct>>,
) {
    for mut transform_lasers in lasers_query.iter_mut() {
        transform_lasers.translation.y += 450.0 * time.delta_secs();
    }
}

pub fn collision_lasers_player_with_enemies (
    mut lasers_query: Query<(&Transform, Entity), With<LasersPlayerStruct>>,
    mut enemies_l1: Query<(&Transform, Entity), With<EnemiesStructInLevel1>>,
    mut enemies_l2: Query<(&Transform, Entity), With<EnemiesStructInLevel2>>,
    mut enemies_l3: Query<(&Transform, Entity), With<EnemiesStructInLevel3>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    for (lasers_tr, lasers_entity) in lasers_query.iter_mut() {
        let enemies_all_levels = enemies_l1.iter_mut()
            .chain(enemies_l2.iter_mut())
            .chain(enemies_l3.iter_mut());

        for (enemies, enemies_entity) in enemies_all_levels {
            let collision = lasers_tr.translation.y > enemies.translation.y
                && lasers_tr.translation.y - 30.0 < enemies.translation.y
                && (lasers_tr.translation.x - enemies.translation.x).abs() < 30.0;

            if collision {
                commands.entity(enemies_entity).try_despawn();
                commands.entity(lasers_entity).try_despawn();
                commands.spawn((
                    AudioPlayer::new(
                        asset_server.load("sounds/collision_of_the_player's_laser_with_the_enemy.ogg")
                    ),
                    PlaybackSettings::ONCE,
                ));
                break;
            }
        }
    }
}
