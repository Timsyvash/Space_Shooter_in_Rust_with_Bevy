use bevy::prelude::*;
use crate::game::GameplayObject;

#[derive(Component)]
pub struct PlayerStruct;

pub fn load_players(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn keys_for_players(asset_server: Res<AssetServer>,
key_code: Res<ButtonInput<KeyCode>>, mut player_query: Query<(&mut Transform, &mut Sprite), With<PlayerStruct>>) {
    for (mut t, mut texture) in player_query.iter_mut() {
        if key_code.just_pressed(KeyCode::KeyD) || key_code.just_pressed(KeyCode::ArrowRight) {
            t.translation.x += 25.0;
            texture.image = asset_server.load("images/players/playerRight.png");
        }
        if key_code.just_pressed(KeyCode::KeyA) || key_code.just_pressed(KeyCode::ArrowLeft) {
            t.translation.x -= 25.0;
            texture.image = asset_server.load("images/players/playerLeft.png");
        }
    }
}

pub fn borders_for_player(mut player_query: Query<&mut Transform, With<PlayerStruct>>) {
    for mut t in player_query.iter_mut() {
        if t.translation.x >= 475.0 {
            t.translation.x = 475.0;
        } else if t.translation.x <= -475.0 {
            t.translation.x = -475.0;
        }
    }
}