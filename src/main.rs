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
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 4.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Sphere::new(0.1)),
        material: materals.add(StandardMaterial::default()),
        ..default()
    });

    commands.spawn((
        MaterialMeshBundle {
            mesh: meshes.add(Sphere::new(0.05)),
            material: materals.add(StandardMaterial {
                base_color: Color::srgb(1.0, 0.0, 0.0),
                ..default()
            }),
            ..default()
        },
        Marker,
    ));
}

fn update(
    buttons: Res<ButtonInput<GamepadButton>>,
    axis: Res<Axis<GamepadAxis>>,
    pads: Res<Gamepads>,
    mut tfs: Query<&mut Transform, With<Marker>>,
) {
    let mut offset = Vec2::ZERO;

    for gamepad in pads.iter() {
        // Dpad
        let dpad_up = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadUp,
        });
        let dpad_down = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadDown,
        });
        let dpad_left = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadLeft,
        });
        let dpad_right = buttons.pressed(GamepadButton {
            gamepad,
            button_type: GamepadButtonType::DPadRight,
        });

        // Right and up are +
        let dpad_x = (dpad_right as isize as f32) - (dpad_left as isize as f32);
        let dpad_y = (dpad_up as isize as f32) - (dpad_down as isize as f32);

        offset += Vec2::new(dpad_x, dpad_y);

        // Joystick
        let axis_x = axis
            .get(GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickX,
            })
            .unwrap();
        let axis_y = axis
            .get(GamepadAxis {
                gamepad,
                axis_type: GamepadAxisType::LeftStickY,
            })
            .unwrap();
        offset += Vec2::new(axis_x, axis_y);
    }

    let mut tf = tfs.single_mut();
    tf.translation = offset.extend(0.1);
}
