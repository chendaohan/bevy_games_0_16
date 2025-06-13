mod camera_controller;

use std::f32;

use bevy::{
    core_pipeline::prepass::DepthPrepass,
    pbr::decal::{ForwardDecal, ForwardDecalMaterial, ForwardDecalMaterialExt},
    prelude::*,
};
use camera_controller::{CameraController, CameraControllerPlugin};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(AssetPlugin {
                file_path: "assets/forward_decals".into(),
                ..default()
            }),
            MeshPickingPlugin,
            CameraControllerPlugin,
        ))
        .insert_resource(MeshPickingSettings {
            require_markers: true,
            ..default()
        })
        .add_systems(Startup, setup)
        .add_systems(Update, draw_forward_decal_gizmo)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    mut decal_standard_materials: ResMut<Assets<ForwardDecalMaterial<StandardMaterial>>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn the forward decal
    commands
        .spawn((
            Name::new("Decal"),
            ForwardDecal,
            MeshMaterial3d(decal_standard_materials.add(ForwardDecalMaterial {
                base: StandardMaterial {
                    base_color_texture: Some(asset_server.load("uv_checker_bw.png")),
                    ..default()
                },
                extension: ForwardDecalMaterialExt {
                    depth_fade_factor: 1.0,
                },
            })),
            Transform::from_scale(Vec3::splat(4.0)),
            Pickable::default(),
        ))
        .observe(drag_forward_decal);

    commands.spawn((
        Name::new("Camera"),
        Camera3d::default(),
        CameraController::default(),
        DepthPrepass, // Must enable the depth prepass to render forward decals
        Transform::from_xyz(1.0, 1.5, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        MeshPickingCamera,
        Pickable::default(),
    ));

    let white_material = standard_materials.add(Color::WHITE);

    commands.spawn((
        Name::new("Floor"),
        Mesh3d(meshes.add(Rectangle::from_length(10.0))),
        MeshMaterial3d(white_material.clone()),
        Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
    ));

    // Spawn a few cube with random rotations to showcase how the decals behave with non-flat geometry
    let num_obs = 10;
    let mut rng = ChaCha8Rng::seed_from_u64(19878367467713);
    for i in 0..num_obs {
        for j in 0..num_obs {
            let rotation_axis: [f32; 3] = rng.random();
            let rotation_vec: Vec3 = rotation_axis.into();
            let rotation: u32 = rng.random_range(0..360);
            let transform = Transform::from_xyz(
                (-num_obs + 1) as f32 / 2.0 + i as f32,
                -0.2,
                (-num_obs + 1) as f32 / 2.0 + j as f32,
            )
            .with_rotation(Quat::from_axis_angle(
                rotation_vec.normalize_or_zero(),
                (rotation as f32).to_radians(),
            ));

            commands.spawn((
                Mesh3d(meshes.add(Cuboid::from_length(0.6))),
                MeshMaterial3d(white_material.clone()),
                transform,
            ));
        }
    }

    commands.spawn((
        Name::new("Light"),
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

fn drag_forward_decal(
    trigger: Trigger<Pointer<Drag>>,
    mut decals: Query<&mut Transform, With<ForwardDecal>>,
    mouse: Res<ButtonInput<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        if let Ok(mut transform) = decals.get_mut(trigger.target) {
            transform.translation.y += -trigger.delta.y * 0.01;
        }
    }
}

fn draw_forward_decal_gizmo(
    mut gizmos: Gizmos,
    decals: Query<&GlobalTransform, With<ForwardDecal>>,
) {
    for transform in decals {
        let scale = transform.scale();
        let size = vec2(scale.x, scale.z);
        gizmos.axes(transform.compute_transform(), 0.2);
        let mut isometry = transform.to_isometry();
        isometry.rotation *= Quat::from_rotation_x(f32::consts::FRAC_PI_2);
        gizmos.rect(isometry, size, Color::Srgba(Srgba::BLUE));
    }
}
