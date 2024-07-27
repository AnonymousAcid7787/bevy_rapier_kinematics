mod math_utils;
mod ik;
mod arm;
mod physics;
mod testing;
mod chain;
mod node;
mod iterator;

use bevy_flycam::{FlyCam, MovementSettings, NoCameraPlayerPlugin};
use bevy_rapier3d::na::{Isometry3, Vector3};
use bevy_rapier3d::plugin::RapierPhysicsPlugin;
use bevy_rapier3d::prelude::{Collider, RigidBody};
use bevy_rapier3d::render::RapierDebugRenderPlugin;
use bevy::math::Vec3;
use bevy::prelude::*;
use chain::SerialKChain;
use ik::CyclicIKSolver;
use k::connect;
use node::{KJointType, KNodeBuilder};

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins,
        RapierPhysicsPlugin::<()>::default(),
        RapierDebugRenderPlugin::default(),
    ))
        .add_systems(Startup, startup)
        .add_systems(Update, update);

    //flycam stuff
    app.add_plugins(NoCameraPlayerPlugin);
    app.world_mut().resource_mut::<MovementSettings>()
        .speed = 2.5;

    app.run();
}

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    //camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0., 2., 2.)
                .looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam,
        Collider::ball(0.1)
    ));
    //ground
    commands.spawn((
        RigidBody::Fixed,
        Collider::cuboid(10., 0.1, 10.),
        Transform::from_xyz(0., -5., 0.),
    ));

    //test object
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Sphere::new(0.1)),
            material: materials.add(Color::linear_rgb(1., 0., 0.)),
            ..default()
        },
        TestObject,
    ));
}

#[derive(Component)]
pub struct TestObject;

pub fn update(
    mut gizmos: Gizmos,
    keys: Res<ButtonInput<KeyCode>>,
    mut target_q: Query<&mut Transform, With<TestObject>>,
    time: Res<Time>,
) {
    let mut targ_transform = target_q.get_single_mut().ok().unwrap();

    let x_movement = (keys.pressed(KeyCode::ArrowRight) as i8 - keys.pressed(KeyCode::ArrowLeft) as i8) as f32;
    let y_movement = (keys.pressed(KeyCode::KeyU) as i8 - keys.pressed(KeyCode::KeyJ) as i8) as f32;
    let z_movement = (keys.pressed(KeyCode::ArrowDown) as i8 - keys.pressed(KeyCode::ArrowUp) as i8) as f32;
    targ_transform.translation += Vec3::new(x_movement, y_movement, z_movement) * time.delta_seconds();

    let base = KNodeBuilder::new()
        .joint_type(KJointType::Revolute { axis: Vector3::y_axis() })
        .build();
    let mut previous = base.clone();
    for _ in 0..5 {
        let new_joint = KNodeBuilder::new()
            .joint_type(KJointType::Revolute { axis: Vector3::x_axis() })
            .translation(Vector3::new(0., -1., 0.).into())
            .build();
        connect![previous => new_joint];
        previous = new_joint.clone();
    }

    let mut chain = SerialKChain::from_root(&base);
    let solver = CyclicIKSolver {
        allowable_target_distance: 0.1,
        allowable_target_angle: 1f32.to_radians(),
        max_iterations: 1
    };
    let solver_result = solver.solve(&mut chain, Isometry3 {
        rotation: targ_transform.rotation.into(),
        translation: targ_transform.translation.into(),
    });
    chain.update_world_transforms();

    let mut prev = Vec3::ZERO;
    for joint in chain.iter_joints() {
        let joint_pos: Vec3 = joint.world_transform().unwrap().translation.into();
        gizmos.sphere(
            joint_pos,
            default(),
            0.05,
            Color::linear_rgb(0., 1., 0.)
        );
        gizmos.line(prev, joint_pos, Color::linear_rgb(0., 0., 1.));
        prev = joint_pos;
    }

    if solver_result.is_err() {
        println!("{}", solver_result.err().unwrap());
    }
}
