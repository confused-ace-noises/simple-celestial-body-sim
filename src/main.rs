use std::fs;

use bevy::core_pipeline::Skybox;
use bevy::prelude::*;
use celestial_body_sim::cel_body::{init_cel_bodies, CelBody};
use celestial_body_sim::cli::{BodyBuilder, Cli};
use celestial_body_sim::{camera::*, G};
use clap::Parser;

#[derive(Resource)]
struct JsonDataPath(String);

fn main() {
    let cli = Cli::parse();




    App::new()
        .insert_resource(JsonDataPath(cli.path))
        .add_plugins(DefaultPlugins)
        .add_plugins(CameraControllerPlugin)
        .add_systems(
            Startup,
            (
                setup_bodies,
                init_cel_bodies.after(setup_bodies),
                //setup_skybox.after(init_cel_bodies),
                setup_camera.after(init_cel_bodies),
            ),
        )
        .add_systems(Update, (gravity, update_position.after(gravity)))
        .run();
}

fn setup_bodies(
    mut commands: Commands,
    json_path: Res<JsonDataPath>
) {
    let x = fs::read_to_string(json_path.0.clone()).expect("failed to read path");
    let json: Vec<BodyBuilder> = serde_json::from_str(&x).expect("failed to parse json");

    let bodies = json.into_iter().map(|x| {
        x.to_cel_body()
    }).collect::<Vec<_>>();
    

    for cel_body in bodies.iter() {
        commands.spawn(cel_body.clone());
    }
}

fn setup_camera(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn((
        Camera3d::default(),
        CameraController::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::new(149_000_000.0, 0.0, 0.0), Vec3::Y),
        Skybox {
            // image: asset_server.load("assets/skybox/NightSkyHDRI012_8K-HDR.exr"),
            image: asset_server.load("skybox/cubemap_toktx90.ktx2"),
            brightness: 100.0,
            ..Default::default()
        },
    ));
}

fn update_position(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut obj_query: Query<(&mut Transform, &mut CelBody)>,
) {
    if input.pressed(KeyCode::Space) {
        for mut obj in obj_query.iter_mut() {
            obj.1.calc_movement(&time, &mut obj.0);
        }
    }

    for obj in obj_query.iter() {
        println!("{:?}", obj.0)
    }
}

fn gravity(input: Res<ButtonInput<KeyCode>>, mut query: Query<(&Transform, &mut CelBody)>) {
    if input.pressed(KeyCode::Space) {
        let mut bodies: Vec<(&Transform, Mut<'_, CelBody>)> = query.iter_mut().collect();

        // Create a vector to store the forces for each body
        let mut forces: Vec<Vec3> = vec![Vec3::ZERO; bodies.len()];

        // First pass: calculate the forces between bodies
        for (index_1, (trans_1, body_1)) in bodies.iter().enumerate() {
            for (index_2, (trans_2, body_2)) in bodies.iter().enumerate() {
                if index_1 != index_2 {
                    let direction = trans_2.translation - trans_1.translation;
                    let distance_squared = direction.length_squared().max(1.0e-11); // Prevent division by zero

                    let force_magnitude =
                        (G * Into::<f32>::into(body_1.mass) * Into::<f32>::into(body_2.mass))
                            / distance_squared;
                    let direction_normalized = direction.normalize();
                    forces[index_1] += force_magnitude * direction_normalized; // Accumulate the force
                }
            }
        }

        // Second pass: update accelerations based on the accumulated forces
        for (index, (_, ref mut body_1)) in bodies.iter_mut().enumerate() {
            body_1.acceleration = forces[index] / Into::<f32>::into(body_1.mass);
            // Apply the calculated force
        }
    }
}
