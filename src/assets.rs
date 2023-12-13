use bevy::prelude::*;

use crate::settings::Settings;

#[derive(Resource)]
pub struct OthelloAssets {
    pub stone_black: Handle<ColorMaterial>,
    pub stone_white: Handle<ColorMaterial>,
    pub stone_shape: Handle<Mesh>,
}

pub fn setup_assets(
    mut commands: Commands,
    settings: Res<Settings>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands.insert_resource(OthelloAssets {
        stone_black: materials.add(settings.stone_black_color.into()),
        stone_white: materials.add(settings.stone_white_color.into()),
        stone_shape: meshes.add(Mesh::from(shape::Circle {
            radius: settings.stone_size().x / 2.0,
            ..default()
        })),
    });
}
