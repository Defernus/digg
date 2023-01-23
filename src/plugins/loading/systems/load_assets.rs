use crate::plugins::loading::resources::{GameAssets, PhysicsObject};
use bevy::{asset::AssetPath, prelude::*};

fn load_scene_with_physics<'a>(
    path: impl Into<AssetPath<'a>>,
    asset_server: &AssetServer,
) -> PhysicsObject {
    let scene_h: Handle<Scene> = asset_server.load(path);

    PhysicsObject {
        scene: scene_h,
        ..default()
    }
}

pub fn load_assets(
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut commands: Commands,
) {
    let game_assets = GameAssets {
        main_font: asset_server.load("fonts/roboto.ttf"),
        default_material: materials.add(StandardMaterial {
            base_color: Color::rgb(1.0, 1.0, 1.0).into(),
            perceptual_roughness: 1.,
            metallic: 0.,
            reflectance: 0.,
            ..default()
        }),
        debug_item_mesh: meshes.add(Mesh::from(shape::Cube { size: 0.2 })),

        tree_object: load_scene_with_physics("models/tree.glb#Scene0", &asset_server),
        branch_object: load_scene_with_physics("models/branch.glb#Scene0", &asset_server),
    };

    commands.insert_resource(game_assets);
}
