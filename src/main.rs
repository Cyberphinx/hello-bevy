use bevy::{prelude::*, utils::FloatOrd};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>,
}

mod bullet;
mod target;
mod tower;

pub use bullet::*;
pub use target::*;
pub use tower::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Inspector setup
        .add_plugins(WorldInspectorPlugin::new())
        // Our systems
        .add_plugins(TowerPlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(BulletPlugin)
        .add_systems(
            Startup,
            (spawn_basic_scene, spawn_camera, spawn_light, asset_loading),
        )
        .run();
}

fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2_500_000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(3.0, 8.0, 3.0),
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
            mesh: meshes.add(Mesh::from(Cuboid::new(0.5, 1.0, 0.5))),
            material: materials.add(Color::GRAY),
            transform: Transform::from_xyz(0.0, 0.25, 0.0),
            ..default()
        })
        .insert(Tower {
            shooting_timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            ..default()
        })
        .insert(Name::new("Tower"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.4, 0.4))),
            material: materials.add(Color::PINK),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3.0 })
        .insert(Name::new("Target1"));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Cuboid::new(0.4, 0.4, 0.4))),
            material: materials.add(Color::PURPLE),
            transform: Transform::from_xyz(-0.4, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3.0 })
        .insert(Name::new("Target2"));
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    let bullet: Handle<Scene> = assets.load("Bullet.glb#Scene0");
    commands.insert_resource(GameAssets {
        bullet_scene: bullet,
    });
}

// #[derive(Component, Reflect)]
// pub struct Ferris;

// fn spawn_gltf(mut commands: Commands, asset_server: Res<AssetServer>) {
//     // note that we have to include the `Scene0` label
//     let my_gltf = asset_server.load("ferris3d.glb#Scene0");

//     // to position our 3d model, simply use the Transform
//     // in the SceneBundle
//     commands
//         .spawn(SceneBundle {
//             scene: my_gltf,
//             transform: Transform::from_xyz(1.0, 0.0, 0.0),
//             ..Default::default()
//         })
//         .insert(Ferris)
//         .insert(Name::new("Ferris"));
// }
