use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, update)
        .run();
}

#[derive(Component)]
struct Marker;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materals: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.1))),
        MeshMaterial3d(materals.add(StandardMaterial::default())),
    ));

    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.05))),
        MeshMaterial3d(materals.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        })),
        Marker,
    ));
}

fn update(pads: Query<&Gamepad>, mut tfs: Query<&mut Transform, With<Marker>>) {
    let mut offset = Vec2::ZERO;

    for gamepad in pads.iter() {
        // Dpad
        let dpad_up = gamepad.pressed(GamepadButton::DPadUp);
        let dpad_down = gamepad.pressed(GamepadButton::DPadDown);
        let dpad_left = gamepad.pressed(GamepadButton::DPadLeft);
        let dpad_right = gamepad.pressed(GamepadButton::DPadRight);

        // Right and up are +
        let dpad_x = (dpad_right as isize as f32) - (dpad_left as isize as f32);
        let dpad_y = (dpad_up as isize as f32) - (dpad_down as isize as f32);

        offset += Vec2::new(dpad_x, dpad_y);

        // Joystick
        let axis_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        let axis_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
        offset += Vec2::new(axis_x, axis_y);
    }

    let mut tf = tfs.single_mut();
    tf.translation = offset.extend(0.1);
}
