use bevy::{pbr::NotShadowCaster, prelude::*, utils::FloatOrd};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::{highlight::Highlight, selection::PickSelection, *};

#[derive(Resource)]
pub struct GameAssets {
    tower_base_scene: Handle<Scene>,
    tomato_tower_scene: Handle<Scene>,
    tomato_scene: Handle<Scene>,
    target_scene: Handle<Scene>,
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
        // Mod Picking
        .add_plugins(DefaultPickingPlugins)
        // Our systems
        .add_plugins(TowerPlugin)
        .add_plugins(TargetPlugin)
        .add_plugins(BulletPlugin)
        .add_systems(PreStartup, asset_loading)
        .add_systems(Startup, (spawn_basic_scene, spawn_camera, spawn_light))
        .add_systems(Update, camera_controls)
        .run();
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>) {
    commands.insert_resource(GameAssets {
        tower_base_scene: assets.load("TowerBase.glb#Scene0"),
        tomato_tower_scene: assets.load("TomatoTower.glb#Scene0"),
        tomato_scene: assets.load("Tomato.glb#Scene0"),
        target_scene: assets.load("Target.glb#Scene0"),
    });
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

fn spawn_basic_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    game_assets: Res<GameAssets>,
) {
    // The ground plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(Mesh::from(Plane3d::default().mesh().size(15.0, 15.0))),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        })
        .insert(Name::new("Ground"));

    let default_collider_color = materials.add(Color::rgba(0.3, 0.5, 0.3, 0.3));
    let hovered_collider_color = materials.add(Color::rgba(0.2, 0.7, 0.3, 0.9));
    let selected_collider_color = materials.add(Color::rgba(0.3, 0.9, 0.3, 0.9));

    // Tower base slot
    commands
        .spawn(SpatialBundle::from_transform(Transform::from_xyz(
            0.0, 0.8, 0.0,
        )))
        .insert(Name::new("Tower_Base"))
        .insert(meshes.add(Capsule3d::default()))
        .insert(Highlight {
            hovered: Some(highlight::HighlightKind::Fixed(
                hovered_collider_color.clone(),
            )),
            pressed: Some(highlight::HighlightKind::Fixed(
                selected_collider_color.clone(),
            )),
            selected: Some(highlight::HighlightKind::Fixed(
                selected_collider_color.clone(),
            )),
        })
        .insert(default_collider_color)
        .insert(NotShadowCaster)
        .insert(PickableBundle::default())
        .with_children(|commands| {
            commands.spawn(SceneBundle {
                scene: game_assets.tower_base_scene.clone(),
                transform: Transform::from_xyz(0.0, -0.8, 0.0),
                ..default()
            });
        });

    // Target 1
    commands
        .spawn(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-2.0, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3.0 })
        .insert(Name::new("Target1"));

    // Target 2
    commands
        .spawn(SceneBundle {
            scene: game_assets.target_scene.clone(),
            transform: Transform::from_xyz(-0.4, 0.2, 1.5),
            ..default()
        })
        .insert(Target { speed: 0.3 })
        .insert(Health { value: 3.0 })
        .insert(Name::new("Target2"));
}

fn spawn_camera(mut commands: Commands) {
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(Name::new("Camera"));
}

fn camera_controls(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut camera_query: Query<&mut Transform, With<Camera3d>>,
    time: Res<Time>,
) {
    let mut camera = camera_query.single_mut();

    // Calculate the forward direction vector without the y component
    let forward = Vec3::new(camera.forward().x, 0.0, camera.forward().z).normalize();
    // Calculate the left direction vector without the y component
    let left = Vec3::new(camera.left().x, 0.0, camera.left().z).normalize();

    let speed = 3.0;
    let rotate_speed = 0.3;

    if keyboard.pressed(KeyCode::KeyW) {
        camera.translation += forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyS) {
        camera.translation -= forward * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyA) {
        camera.translation += left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyD) {
        camera.translation -= left * time.delta_seconds() * speed;
    }
    if keyboard.pressed(KeyCode::KeyQ) {
        camera.rotate_axis(Vec3::Y, rotate_speed * time.delta_seconds());
    }
    if keyboard.pressed(KeyCode::KeyE) {
        camera.rotate_axis(Vec3::Y, -rotate_speed * time.delta_seconds());
    }
}
