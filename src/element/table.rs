use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::Side;
use crate::shapes::{Ellipse, Flipper, Origin, Table};
use crate::{
    BALL_GROUP, BALL_RADIUS, FLIPPERS_GROUP, FLIPPER_BIG, FLIPPER_SMALL, GUIDE_HEIGHT, RESOLUTION,
    TABLE_GROUP, TABLE_HEIGHT, TABLE_INCLINATION, TABLE_WIDTH, WALL_HEIGHT,
};

fn table(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    let mesh = Mesh::from(Table::new(TABLE_HEIGHT, TABLE_WIDTH, WALL_HEIGHT));
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            transform: Transform::from_rotation(Quat::from_rotation_x(TABLE_INCLINATION)),
            ..default()
        })
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .insert(RigidBody::Fixed)
        .insert(collider)
        .id()
}

fn linear_guide(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    for flipped in [true, false] {
        let rotation = if flipped {
            Quat::from_rotation_y(PI / 2.)
        } else {
            Quat::from_rotation_y(-PI / 2.)
        };
        let mesh = Mesh::from(Rectangle::new(GUIDE_HEIGHT, WALL_HEIGHT));
        let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
        let guide = commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
                ..default()
            })
            .insert(
                Transform::from_rotation(rotation).with_translation(Vec3::new(
                    TABLE_WIDTH / 2. - (BALL_RADIUS + 0.05) * 2.,
                    0.,
                    (TABLE_HEIGHT - GUIDE_HEIGHT) / 2.,
                )),
            )
            .insert(RigidBody::Fixed)
            .insert(collider)
            .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
            .id();
        commands.entity(table).add_child(guide);
    }
}

fn elliptic_guide(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    let mesh = Ellipse {
        rectangle: false,
        center: Origin::MaxXMinZ,
        first_angle: 0.,
        second_angle: PI / 4.,
        resolution: RESOLUTION,
        x: 2.5,
        z: 0.9,
        thickness: WALL_HEIGHT,
    }
    .try_into()
    .unwrap();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let elliptic_guide = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            ..default()
        })
        .insert(
            Transform::from_rotation(Quat::from_rotation_x(PI)).with_translation(Vec3::new(
                TABLE_WIDTH / 2. - (BALL_RADIUS + 0.05) * 2.,
                0.,
                TABLE_HEIGHT / 2. - GUIDE_HEIGHT,
            )),
        )
        .insert(RigidBody::Fixed)
        .insert(collider)
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .id();
    commands.entity(table).add_child(elliptic_guide);
}

fn top_ellipses(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    let mesh = Ellipse {
        rectangle: true,
        center: Origin::MinXMaxZ,
        first_angle: 0.,
        second_angle: PI / 2.,
        resolution: RESOLUTION,
        x: TABLE_WIDTH / 2.,
        z: 0.9,
        thickness: WALL_HEIGHT,
    }
    .try_into()
    .unwrap();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let up = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            ..default()
        })
        .insert(
            Transform::from_rotation(Quat::from_rotation_y(PI)).with_translation(Vec3::new(
                0.,
                0.,
                -TABLE_HEIGHT / 2.,
            )),
        )
        .insert(RigidBody::Fixed)
        .insert(collider)
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .id();
    commands.entity(table).add_child(up);

    // Top right ellipse
    let mesh = Ellipse {
        rectangle: true,
        center: Origin::MinXMaxZ,
        first_angle: 0.,
        second_angle: PI / 2.,
        resolution: RESOLUTION,
        x: TABLE_WIDTH / 2.,
        z: 0.9,
        thickness: WALL_HEIGHT,
    }
    .try_into()
    .unwrap();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let up = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            ..default()
        })
        .insert(
            Transform::from_rotation(Quat::from_rotation_x(PI)).with_translation(Vec3::new(
                0.,
                0.,
                -TABLE_HEIGHT / 2.,
            )),
        )
        .insert(RigidBody::Fixed)
        .insert(collider)
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .id();
    commands.entity(table).add_child(up);
}

fn middle_left_ellipse(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    let ellipse = Ellipse {
        rectangle: true,
        center: Origin::MaxXMaxZ,
        first_angle: PI / 2.,
        second_angle: PI / 8.,
        resolution: RESOLUTION,
        x: 0.8,
        z: 0.3,
        thickness: WALL_HEIGHT,
    };
    // Because of -PI/2 rotation in transform
    // x "is z".
    let x = ellipse.real_z();
    let mesh = ellipse.try_into().unwrap();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let ellipse =
        commands
            .spawn(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
                ..default()
            })
            .insert(
                Transform::from_rotation(Quat::from_rotation_y(-PI / 2.))
                    .with_translation(Vec3::new(-TABLE_WIDTH / 2., 0., 0.)),
            )
            .insert(RigidBody::Fixed)
            .insert(collider)
            .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
            .id();
    commands.entity(table).add_child(ellipse);

    // Upper left flipper
    let angle = -PI / 10.;
    let mesh = Flipper::new(
        0.7,
        FLIPPER_SMALL,
        FLIPPER_BIG,
        WALL_HEIGHT - 0.02,
        RESOLUTION,
    )
    .into();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let position_in_table = Vec3::new(
        -TABLE_WIDTH / 2. + FLIPPER_BIG + x - angle.cos() * FLIPPER_BIG + 0.02,
        0.,
        (PI / 2.).sin() * FLIPPER_BIG + 0.02,
    );

    let rotation = RevoluteJointBuilder::new(Vec3::new(0., 1., 0.))
        .local_anchor1(position_in_table)
        .local_anchor2(Vec3::ZERO)
        .limits([angle, angle + PI / 3.]);
    let upper_left_flipper = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::WHITE),
            transform: Transform::from_translation(position_in_table)
                .with_rotation(Quat::from_rotation_y(angle)),
            ..Default::default()
        })
        .insert(Side::Left)
        .insert(RigidBody::Dynamic)
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(collider)
        .insert(CollisionGroups::new(FLIPPERS_GROUP, BALL_GROUP))
        .insert(Restitution::coefficient(0.3))
        .insert(ImpulseJoint::new(table, rotation))
        .insert(Damping {
            linear_damping: 0.,
            angular_damping: 5.,
        })
        .insert(Ccd::enabled())
        .id();
    commands.entity(table).add_child(upper_left_flipper);
}

fn middle_right_ellipse(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    let ellipse = Ellipse {
        rectangle: true,
        center: Origin::MaxXMaxZ,
        first_angle: PI / 2.,
        second_angle: PI / 8.,
        resolution: RESOLUTION,
        x: 0.8,
        z: 0.3,
        thickness: WALL_HEIGHT,
    };
    // Because of -PI/2 rotation in transform
    // x "is z".
    let z = ellipse.real_x();
    let mesh = ellipse.try_into().unwrap();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let ellipse = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::srgb(0.4, 0.4, 0.4)),
            ..default()
        })
        .insert(
            Transform::from_rotation(Quat::from_rotation_z(PI) * Quat::from_rotation_y(-PI / 2.))
                .with_translation(Vec3::new(
                    TABLE_WIDTH / 2. - (BALL_RADIUS + 0.05) * 2.,
                    0.,
                    (TABLE_HEIGHT - GUIDE_HEIGHT) / 2. - z,
                )),
        )
        .insert(RigidBody::Fixed)
        .insert(collider)
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .id();
    commands.entity(table).add_child(ellipse);
}

fn glass(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    table: Entity,
) {
    let mesh = Cuboid::new(TABLE_WIDTH, 0.05, TABLE_HEIGHT).into();
    let collider = Collider::from_bevy_mesh(&mesh, &ComputedColliderShape::TriMesh).unwrap();
    let glass = commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::NONE),
            transform: Transform::from_translation(Vec3::new(0., WALL_HEIGHT / 2., 0.)),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert(collider)
        .insert(CollisionGroups::new(TABLE_GROUP, BALL_GROUP))
        .id();
    commands.entity(table).add_child(glass);
}

pub(crate) fn setup_table(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    // Table
    let table = table(commands, meshes, materials);

    // Ball starting guide
    linear_guide(commands, meshes, materials, table);

    // Ball starting elliptic guide
    elliptic_guide(commands, meshes, materials, table);

    // Top ellipses
    top_ellipses(commands, meshes, materials, table);

    // Ellipse for middle flippers
    middle_left_ellipse(commands, meshes, materials, table);
    //middle_right_ellipse(commands, meshes, materials, table);

    // Glass on top
    glass(commands, meshes, materials, table);

    table
}
