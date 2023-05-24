use bevy::{
    ecs::event::ManualEventReader,
    input::mouse::MouseMotion,
    prelude::{
        shape, App, Assets, Camera3dBundle, Color, Commands, Component, Events, Input,
        KeyCode, Mesh, PbrBundle, PointLight, PointLightBundle, Quat, Query, Res,
        ResMut, Resource, StandardMaterial, Transform, Vec3, With,
    },
    window::{CursorGrabMode, PrimaryWindow, Window},
    DefaultPlugins, time::Time,
};
use bevy_rapier3d::prelude::{
    Collider, ColliderMassProperties, ExternalForce, NoUserData,
    RapierPhysicsPlugin, RigidBody, Friction, CoefficientCombineRule,
};

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .init_resource::<InputState>()
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_startup_system(setup)
        .add_startup_system(initial_cursor_grab)
        .add_system(player_move)
        .add_system(player_look)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(shape::Plane::from_size(500.0).into()),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Collider::cuboid(500.0, 0.1, 500.0));

    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..Default::default()
    });

    // camera
    commands
        .spawn((
            Camera3dBundle {
                transform: Transform::from_xyz(0.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
                ..Default::default()
            },
            Player,
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::cuboid(0.5, 0.5, 0.5),
            ColliderMassProperties::Mass(2.0),
            Friction {
                coefficient: 2.0,
                combine_rule: CoefficientCombineRule::Average
            },
        ));
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    let window = windows.single();
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                if window.cursor.grab_mode == CursorGrabMode::Locked {
                    match key {
                        KeyCode::W => velocity += forward,
                        KeyCode::S => velocity -= forward,
                        KeyCode::A => velocity -= right,
                        KeyCode::D => velocity += right,
                        KeyCode::Space => velocity += Vec3::Y,
                        KeyCode::LShift => velocity -= Vec3::Y,
                        _ => (),
                    }
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * 1.0 /*sensitivity*/
        }
}

/// Handles looking around if cursor is locked
fn player_look(
    windows: Query<&Window, With<PrimaryWindow>>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query: ResMut<Camera3dBundle>
) {
    let window = windows.single();
        let mut delta_state = state.as_mut();
        for mut transform in query.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor.grab_mode == CursorGrabMode::Locked {
                    // Using smallest of height or width ensures equal vertical and horizontal sensitivity
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (/*sensitivity*/ 0.00012 * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (/*sensitivity*/ 0.00012 * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                // Order is important to prevent unintended roll
                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
}

fn initial_cursor_grab(mut windows: Query<&mut Window, With<PrimaryWindow>>) {
    let mut window = windows.single_mut();
    window.cursor.grab_mode = CursorGrabMode::Locked;
    window.cursor.visible = false;
}