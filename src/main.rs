use std::fs;

use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use celestial_body_sim::cel_body::{init_cel_bodies, Acceleration, Mass, UpdatePosition, Velocity};
use celestial_body_sim::cli::{BodyBuilder, Cli};
use celestial_body_sim::{camera::*, JsonData, G};
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    let bodybuilders = get_bodybuilders(cli.path);

    App::new()
        .insert_resource(JsonData(bodybuilders))
        .add_plugins((DefaultPlugins, CameraControllerPlugin))
        .add_systems(
            Startup,
            (
                init_cel_bodies,
                setup_camera.after(init_cel_bodies),
            ),
        )
        .add_systems(Update, (gravity, update_position.after(gravity)
        ))
        .run();
}

fn setup_camera(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::from_xyz(0.0, 0.0, 0.0).looking_at(Vec3::new(149_000_000.0, 0.0, 0.0), Vec3::Y),
        Skybox {
            // image: asset_server.load("assets/skybox/NightSkyHDRI012_8K-HDR.exr"),
            image: asset_server.load("skybox/cubemap_toktx90.ktx2"),
            brightness: 85.0,
            ..Default::default()
        },
    ));
}

fn update_position(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut to_update_query: Query<(&mut Transform, &mut Velocity, &Acceleration), With<UpdatePosition>>,
) {
    if input.pressed(KeyCode::Space) {
        for mut obj in to_update_query.iter_mut() {
            let delta_time = time.delta_secs();

            obj.1.0 += obj.2.0 * delta_time; // velocity += acceleration * delta_time
            obj.0.translation += obj.1.0 * delta_time; // transform.translation += velocity * delta_time
        }
    }

    // for obj in obj_query.iter() {
    //     println!("{:?}", obj.0)
    // }
}

fn gravity(
    input: Res<ButtonInput<KeyCode>>, 
    mut query: Query<(&Transform, &mut Acceleration, &Mass), With<UpdatePosition>>
) {
    if input.pressed(KeyCode::Space) {
        let mut bodies: Vec<_>= query.iter_mut().collect();

        // Create a vector to store the forces for each body
        let mut forces: Vec<Vec3> = vec![Vec3::ZERO; bodies.len()];

        // First pass: calculate the forces between bodies
        for (index_1, (trans_1, _, mass_1)) in bodies.iter().enumerate() {
            for (index_2, (trans_2, _, mass_2)) in bodies.iter().enumerate() {
                if index_1 != index_2 {
                    let direction = trans_2.translation - trans_1.translation;
                    let distance_squared = direction.length_squared().max(1.0e-11); // Prevent division by zero

                    let force_magnitude =
                        (G * Into::<f32>::into(mass_1.0) * Into::<f32>::into(mass_2.0))
                            / distance_squared;
                    let direction_normalized = direction.normalize();
                    forces[index_1] += force_magnitude * direction_normalized; // Accumulate the force
                }
            }
        }

        // Second pass: update accelerations based on the accumulated forces
        for (index, (_, acceleration_1, mass_1)) in bodies.iter_mut().enumerate() {
            acceleration_1.0 = forces[index] / Into::<f32>::into(mass_1.0);
            // Apply the calculated force
        }
    }
}

pub fn get_bodybuilders(path: String) -> Vec<BodyBuilder> {
    let raw = fs::read_to_string(path).unwrap();
    let json: Vec<BodyBuilder> = serde_json::from_str(raw.as_str()).unwrap();

    json
}

// fn text(
//     mut commands: Commands,
// ) {
//     commands.spawn((
//         Text
//     ))
// }