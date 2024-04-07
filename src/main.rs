use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_systems(Startup, (spawn_basic_scene, spawn_camera, spawn_light))
        // .add_systems(Startup, spawn_gltf)
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    };
    commands.spawn(light).insert(Name::new("Light"));
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Camera"));
}

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        })
        .insert(Name::new("Ground"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5))),
            material: materials.add(Color::rgb(0.67, 0.84, 0.92)),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        })
        .insert(Name::new("Cube"));
}

// fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // note that we have to include the `Scene0` label
//     let my_gltf = asset_server.load("Ferris.glb#Scene0");

//     // to position our 3d model, simply use the Transform
//     // in the SceneBundle
//     commands
//         .spawn(SceneBundle {
//             scene: my_gltf,
//             transform: Transform::from_xyz(0.0, 0.0, 0.0),
//             ..Default::default()
//         })
//         .insert(Name::new("Ferris"));
// }
