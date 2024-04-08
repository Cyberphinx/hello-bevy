use bevy::prelude::*;

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Target {
    pub speed: f32,
}

#[derive(Reflect, Component, Default)]
#[reflect(Component)]
pub struct Health {
    pub value: f32,
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Target>()
            .register_type::<Health>()
            .add_systems(Update, (move_targets, target_death));
    }
}

fn target_death(mut commands: Commands, targets: Query<(Entity, &Health)>) {
    for (ent, health) in &targets {
        if health.value <= 0.0 {
            commands.entity(ent).despawn_recursive();
        }
    }
}

fn move_targets(mut targets: Query<(&Target, &mut Transform)>, time: Res<Time>) {
    for (target, mut transform) in &mut targets {
        transform.translation.x += target.speed * time.delta_seconds();
    }
}
