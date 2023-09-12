#![warn(clippy::cargo_common_metadata)]

use std::f32::consts::PI;

#[cfg(feature = "diagnostic")]
use bevy::diagnostic::*;
use bevy::prelude::*;
use bevy::{log::LogPlugin, window::WindowTheme};
#[cfg(feature = "inspector")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::IntegrationParameters;
use element::Left;
use shapes::Flipper;

mod element;
mod shapes;

/// Ball group
const BALL_GROUP: Group = Group::GROUP_1;
/// Table "fixed" element
const TABLE_GROUP: Group = Group::GROUP_2;
/// Flippers
const FLIPPERS_GROUP: Group = Group::GROUP_32;

const RESOLUTION: usize = 20;
const TABLE_INCLINATION: f32 = 6.5 * PI / 180.;
const TABLE_HEIGHT: f32 = 8.;
const TABLE_WIDTH: f32 = 5.;
const WALL_HEIGHT: f32 = 0.3;
const BALL_RADIUS: f32 = 0.1;
const GUIDE_HEIGHT: f32 = TABLE_HEIGHT - 1.2;
const FLIPPER_BIG: f32 = 0.1;
const FLIPPER_SMALL: f32 = 0.05;
const PAD_DEAD_ZONE: f32 = 0.2;

#[derive(Component)]
struct Ball;

fn main() {
    // Tips from https://bevy-cheatbook.github.io/features/log.html
    // this code is compiled only if debug assertions are enabled (debug mode)
    #[cfg(debug_assertions)]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn,pinball=debug".into(),
    };

    #[cfg(not(debug_assertions))]
    let log_plugin = LogPlugin {
        level: bevy::log::Level::DEBUG,
        filter: "info,wgpu_core=warn,wgpu_hal=warn".into(),
    };

    // Use WinitPlugin ??
    let mut app = App::new();
    let app = app.add_plugins((
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pinball".into(),
                    window_theme: Some(WindowTheme::Dark),
                    ..default()
                }),
                ..default()
            })
            .set(log_plugin),
        RapierPhysicsPlugin::<NoUserData>::default(),
    ));

    #[cfg(feature = "inspector")]
    app.add_plugins(WorldInspectorPlugin::default());

    #[cfg(feature = "diagnostic")]
    app.add_plugins((
        LogDiagnosticsPlugin::default(),
        EntityCountDiagnosticsPlugin::default(),
        FrameTimeDiagnosticsPlugin::default(),
        SystemInformationDiagnosticsPlugin::default(),
    ));

    #[cfg(feature = "debug")]
    app.add_plugins(RapierDebugRenderPlugin::default());

    #[cfg(feature = "camera")]
    app.add_plugins(bevy_panorbit_camera::PanOrbitCameraPlugin);

    app.add_systems(Startup, setup_camera_and_physics)
        .add_systems(Startup, setup)
        .add_systems(Update, impulse_ball)
        .add_systems(Update, flip)
        .run();
}

fn setup_camera_and_physics(mut commands: Commands, mut rapier_context: ResMut<RapierContext>) {
    let integration = IntegrationParameters {
        max_ccd_substeps: 1,
        ..Default::default()
    };
    rapier_context.integration_parameters = integration;

    let mut _entity_commands = commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0., 8., 7.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    #[cfg(feature = "camera")]
    _entity_commands.insert(bevy_panorbit_camera::PanOrbitCamera::default());

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(0., 8., -7.),
        ..default()
    });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let table = element::setup_table(&mut commands, &mut meshes, &mut materials);

    // Ball
    let mesh = Mesh::try_from(shape::Icosphere {
        radius: BALL_RADIUS,
        subdivisions: 5,
    })
    .unwrap();
    let ball = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0., 0., 1.).into()),
            ..default()
        })
        .insert(RigidBody::Dynamic)
        .insert(Collider::ball(0.1))
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Restitution::coefficient(0.7))
        //.insert(ColliderMassProperties::Density(7.86))
        .insert(Damping {
            linear_damping: 0.2,
            angular_damping: 0.2,
        })
        .insert(CollisionGroups::new(BALL_GROUP, Group::all() - BALL_GROUP))
        .insert(TransformBundle::from(Transform::from_xyz(
            TABLE_WIDTH / 2. - (BALL_RADIUS + 0.01),
            BALL_RADIUS + 0.01,
            TABLE_HEIGHT / 2. - (BALL_RADIUS + 0.01),
        )))
        .insert(Ccd::enabled())
        .insert(Ball)
        .id();
    commands.entity(table).add_child(ball);
}

fn impulse_ball(
    keyboard: Res<Input<KeyCode>>,
    ball: Query<Entity, With<Ball>>,
    mut commands: Commands,
) {
    if let Ok(ball) = ball.get_single() {
        if keyboard.pressed(KeyCode::Space) {
            let impulse = ExternalImpulse {
                impulse: Vec3::new(0., 0., -0.01),
                torque_impulse: Vec3::ZERO,
            };
            commands.entity(ball).insert(impulse);
        }
    }
}

fn flip(keyboard: Res<Input<KeyCode>>, query: Query<Entity, With<Left>>, mut commands: Commands) {
    for entity in &mut query.iter() {
        if keyboard.pressed(KeyCode::ControlLeft) {
            let impulse = ExternalImpulse {
                impulse: Vec3::new(0., 0., -0.2),
                torque_impulse: Vec3::ZERO,
            };

            commands.entity(entity).insert(impulse);
        };
    }
}

fn test_shape(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = Flipper::new(5., 0.3, 1., 1., 10).into();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::GRAY.into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(collider);
}
