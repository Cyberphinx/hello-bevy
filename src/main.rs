use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .register_type::<Tower>()
        .add_systems(Startup, (spawn_basic_scene, spawn_camera, spawn_light, asset_loading))
        .add_systems(Update, tower_shooting)
        .add_systems(Update, bullet_despawn)
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

#[derive(Component, Reflect)]
pub struct Tower {
    shooting_timer: Timer,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Lifetime {
    timer: Timer,
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
        })
        .insert(Name::new("Tower"));
}

fn tower_shooting(
    mut commands: Commands,
    mut towers: Query<&mut Tower>,
    bullet_assets: Res<GameAssets>,
    time: Res<Time>,
) {
    for mut tower in &mut towers {
        tower.shooting_timer.tick(time.delta());
        if tower.shooting_timer.just_finished() {
            let spawn_transform =
                Transform::from_xyz(0.0, 0.7, 0.6).with_rotation(Quat::from_rotation_y(-PI / 2.0));

            commands
                .spawn(SceneBundle{
                    scene: bullet_assets.bullet_scene.clone(),
                    transform: spawn_transform,
                    ..Default::default()
                })
                .insert(Lifetime {
                    timer: Timer::from_seconds(0.5, TimerMode::Once),
                })
                .insert(Name::new("Bullet"));
        }
    }
}

fn bullet_despawn(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut bullets {
        lifetime.timer.tick(time.delta());
        if lifetime.timer.just_finished(){
            // always use despawn_recursive, since it will despawn all children of the entity
            commands.entity(entity).despawn_recursive();
        }
    }
}

#[derive(Resource)]
pub struct GameAssets {
    bullet_scene: Handle<Scene>
}

fn asset_loading(mut commands: Commands, assets: Res<AssetServer>){
    let bullet: Handle<Scene> = assets.load("Bullet.glb#Scene0");
    commands.insert_resource(GameAssets{ bullet_scene: bullet });
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
