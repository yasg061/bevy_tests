// use bevy::prelude::*;

// fn main() {
//     App::new()
//         .insert_resource(AmbientLight {
//             color: Color::WHITE,
//             brightness: 1.0 / 5.0f32,
//         })
//         .add_plugins(DefaultPlugins)
//         .add_startup_system(setup)
//         .add_system(animate_light_direction)
//         .run();
// }

// fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
//     commands.spawn_scene(asset_server.load("models/eva.gltf#Scene0"));
//     commands.spawn_bundle(PerspectiveCameraBundle {
//         transform: Transform::from_xyz(0.7, 0.7, -5.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
//         ..default()
//     });
//     const HALF_SIZE: f32 = 1.0;
//     commands.spawn_bundle(DirectionalLightBundle {
//         directional_light: DirectionalLight {
//             shadow_projection: OrthographicProjection {
//                 left: -HALF_SIZE,
//                 right: HALF_SIZE,
//                 bottom: -HALF_SIZE,
//                 top: HALF_SIZE,
//                 near: -10.0 * HALF_SIZE,
//                 far: 10.0 * HALF_SIZE,
//                 ..default()
//             },
//             shadows_enabled: true,
//             ..default()
//         },
//         ..default()
//     });
// }

// fn animate_light_direction(
//     time: Res<Time>,
//     mut query: Query<&mut Transform, With<DirectionalLight>>,
// ) {
//     for mut transform in query.iter_mut() {
//         transform.rotation = Quat::from_euler(
//             EulerRot::ZYX,
//             0.0,
//             time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
//             -std::f32::consts::FRAC_PI_4,
//         );
//     }
// }



use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(AmbientLight {
            color: Color::WHITE,
            brightness: 1.0/ 5.0f32,
        })
        .add_startup_system(setup)
        .add_system(setup_scene_once_loaded)
        .add_system(keyboard_animation_control)
        .add_system(animate_light_direction)
        .run();
}

struct Animations(Vec<Handle<AnimationClip>>);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // Insert a resource with the current scene information
    commands.insert_resource(Animations(vec![
        asset_server.load("models/eva.gltf#Animation5"),
        asset_server.load("models/eva.gltf#Animation4"),
        asset_server.load("models/eva.gltf#Animation3"),
        asset_server.load("models/eva.gltf#Animation2"),
        asset_server.load("models/eva.gltf#Animation1"),
        asset_server.load("models/eva.gltf#Animation0"),
    ]));

    // Camera
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0.7, 0.7, -5.0).looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::Y),
        ..Default::default()});
    const HALF_SIZE: f32 = 1.0;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadow_projection: OrthographicProjection {
                left: -HALF_SIZE,
                right: HALF_SIZE,
                bottom: -HALF_SIZE,
                top: HALF_SIZE,
                near: -10.0 * HALF_SIZE,
                far: 10.0 * HALF_SIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        ..default()
        
    });

    // Plane
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane { size: 500000.0 })),
        material: materials.add(Color::rgb(0.0, 0.0, 0.246).into()),
        ..default()
    });

    // // Light
    // commands.spawn_bundle(DirectionalLightBundle {
    //     transform: Transform::from_rotation(Quat::from_euler(
    //         EulerRot::ZYX,
    //         0.0,
    //         1.0,
    //         -std::f32::consts::FRAC_PI_4,
    //     )),
    //     directional_light: DirectionalLight {
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     ..default()
    // });

    // Fox
    scene_spawner.spawn(asset_server.load("models/eva.gltf#Scene0"));

    println!("Animation controls:");
    println!("  - spacebar: play / pause");
    println!("  - arrow up / down: speed up / slow down animation playback");
    println!("  - arrow left / right: seek backward / forward");
    println!("  - return: change animation");
}

// Once the scene is loaded, start the animation
fn setup_scene_once_loaded(
    animations: Res<Animations>,
    mut player: Query<&mut AnimationPlayer>,
    mut done: Local<bool>,
) {
    if !*done {
        if let Ok(mut player) = player.get_single_mut() {
            player.play(animations.0[0].clone_weak()).repeat();
            *done = true;
        }
    }
}

fn keyboard_animation_control(
    keyboard_input: Res<Input<KeyCode>>,
    mut animation_player: Query<&mut AnimationPlayer>,
    animations: Res<Animations>,
    mut current_animation: Local<usize>,
) {
    if let Ok(mut player) = animation_player.get_single_mut() {
        if keyboard_input.just_pressed(KeyCode::Space) {
            if player.is_paused() {
                player.resume();
            } else {
                player.pause();
            }
        }

        if keyboard_input.just_pressed(KeyCode::Up) {
            let speed = player.speed();
            player.set_speed(speed * 1.2);
        }

        if keyboard_input.just_pressed(KeyCode::Down) {
            let speed = player.speed();
            player.set_speed(speed * 0.8);
        }

        if keyboard_input.just_pressed(KeyCode::Left) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed - 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Right) {
            let elapsed = player.elapsed();
            player.set_elapsed(elapsed + 0.1);
        }

        if keyboard_input.just_pressed(KeyCode::Return) {
            *current_animation = (*current_animation + 1) % animations.0.len();
            player
                .play(animations.0[*current_animation].clone_weak())
                .repeat();
        }
    }
}


fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
) {
    for mut transform in query.iter_mut() {
        transform.rotation = Quat::from_euler(
            EulerRot::ZYX,
            0.0,
            time.seconds_since_startup() as f32 * std::f32::consts::TAU / 10.0,
            -std::f32::consts::FRAC_PI_4,
        );
    }
}