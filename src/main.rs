use bevy::{
    ecs::event::ManualEventReader,
    input::{mouse::MouseMotion},
    prelude::{
        shape, App, Assets, Camera3dBundle, Color, Commands, Component, EulerRot,
        Events, Input, KeyCode, Mesh, MouseButton, PbrBundle, PointLight, PointLightBundle, Quat,
        Query, Res, ResMut, Resource, StandardMaterial, Transform, Vec3, With,
    },
    window::{CursorGrabMode, PrimaryWindow, Window},
    DefaultPlugins,
};

#[derive(Resource, Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
}

#[derive(Component)]
pub struct Player;

fn main() {
    App::new()
        .init_resource::<InputState>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(player_movement)
        .add_system(cursor_lock)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..Default::default()
    });
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
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        Player,
    ));
}

fn player_movement(
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut player: Query<&mut Transform, With<Player>>,
    keyboard: Res<Input<KeyCode>>,
    windows: Query<&mut Window, With<PrimaryWindow>>,
) {
    if windows.single().cursor.grab_mode == CursorGrabMode::Locked {
        let mut transform = player.single_mut();
        for ev in state.reader_motion.iter(&motion) {
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            // Using smallest of height or width ensures equal vertical and horizontal sensitivity
            pitch -= (0.1 * ev.delta.y).to_radians();
            yaw -= (0.1 * ev.delta.x).to_radians();

            pitch = pitch.clamp(-1.54, 1.54);

            // Order is important to prevent unintended roll
            transform.rotation =
                Quat::from_axis_angle(Vec3::Y, yaw) * Quat::from_axis_angle(Vec3::X, pitch);
        }

        if keyboard.pressed(KeyCode::W) {
            let forward = transform.forward();
            transform.translation.x += forward.x;
            transform.translation.z += forward.z;
        }

        if keyboard.pressed(KeyCode::A) {
            let left = transform.left();
            transform.translation.x += left.x;
            transform.translation.z += left.z;
        }

        if keyboard.pressed(KeyCode::S) {
            let back = transform.back();
            transform.translation.x += back.x;
            transform.translation.z += back.z;
        }

        if keyboard.pressed(KeyCode::D) {
            let right = transform.right();
            transform.translation.x += right.x;
            transform.translation.z += right.z;
        }

        if keyboard.pressed(KeyCode::Space) {
            transform.translation.y += 1.0;
        }

        if keyboard.pressed(KeyCode::LShift) {
            transform.translation.y -= 1.0;
        }
    }
}

fn cursor_lock(
    mut windows: Query<&mut Window, With<PrimaryWindow>>,
    btn: Res<Input<MouseButton>>,
    key: Res<Input<KeyCode>>,
) {
    let mut window = windows.single_mut();

    if btn.just_pressed(MouseButton::Left) {
        window.cursor.grab_mode = CursorGrabMode::Locked;
        window.cursor.visible = false;
    }

    if key.just_pressed(KeyCode::Escape) {
        window.cursor.grab_mode = CursorGrabMode::None;
        window.cursor.visible = true;
    }
}
