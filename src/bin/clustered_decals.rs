#[path ="../camera_controller.rs"]
mod camera_controller;

use bevy::{pbr::decal::clustered::ClusteredDecal, prelude::*};

use crate::camera_controller::{CameraController, CameraControllerPlugin};

fn main() -> AppExit {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "assets/clustered_decals".into(),
                ..default()
            }),
            CameraControllerPlugin,
            MeshPickingPlugin,
        ))
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, draw_decals_gizmo)
        .run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::from_xyz(2.0, 0.0, 7.0).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(1.0, 5.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    let cuboid = meshes.add(Cuboid::new(1.0, 4.0, 6.0));
    let white = materials.add(StandardMaterial::from_color(Color::WHITE));
    let cuboid_0 = commands
        .spawn((
            Mesh3d(cuboid.clone()),
            MeshMaterial3d(white.clone()),
            Pickable::default(),
        ))
        .id();

    let cuboid_1 = commands
        .spawn((
            Mesh3d(cuboid),
            MeshMaterial3d(white),
            Transform::from_xyz(-3.0, 0.0, 0.0),
            Pickable::default(),
        ))
        .id();

    commands.spawn(
        Observer::new(drag_cuboid)
            .with_entity(cuboid_0)
            .with_entity(cuboid_1),
    );

    commands.spawn((
        ClusteredDecal {
            image: asset_server.load("icon.png"),
            tag: 1,
        },
        Transform::from_xyz(2.0, 0.0, 0.0)
            .with_scale(vec3(2.0, 2.0, 6.0))
            .looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn drag_cuboid(
    trigger: Trigger<Pointer<Drag>>,
    mut cuboids: Query<&mut Transform, With<Mesh3d>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Ok(mut transform) = cuboids.get_mut(trigger.target) {
            transform.translation.x += trigger.delta.x * 0.01;
        }
    }
}

fn draw_decals_gizmo(mut gizmos: Gizmos, decals: Query<&GlobalTransform, With<ClusteredDecal>>) {
    for transform in decals {
        let transform = transform.compute_transform();
        gizmos.cuboid(transform, Color::WHITE);
        gizmos.axes(transform, 0.5);
    }
}
