use std::f32::consts::PI;

use bevy::{prelude::*, render::mesh::SphereMeshBuilder};

use crate::sci_float::SciFloat;

#[derive(Debug, Component, Clone)]
pub struct CelBody {
    pub position: Vec3,
    pub radius: SciFloat,
    pub color: Color,
    pub light: bool,
    pub velocity: Vec3,
    pub acceleration: Vec3,
    pub mass: SciFloat,
    pub density: SciFloat // kg/m³
}
impl CelBody {
    pub fn new(position: Vec3, radius: SciFloat, color: Color, light: bool, velocity: Vec3, acceleration: Vec3, mass: SciFloat) -> CelBody {
        CelBody {
            position: position,
            radius,
            color,
            light,
            velocity: velocity,
            acceleration: acceleration,
            mass,
            density: Into::<SciFloat>::into(4.0/3.0)* PI.into() * radius*radius*radius,
        }
    }

    pub fn calc_movement(&mut self, time: &Res<Time>, transform: &mut Transform) {
        let delta_time = time.delta_secs();

        self.velocity += self.acceleration * delta_time;

        transform.translation += self.velocity * delta_time;
        self.position += self.velocity * delta_time;
    }
}

pub fn init_cel_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    bodies_query: Query<(Entity, &CelBody), With<CelBody>>,
) {
    for (entity, body) in bodies_query.iter() {
        let mut handle = commands.entity(entity);

        if !body.light {
            handle.insert((
                Mesh3d(meshes.add(SphereMeshBuilder::new(
                    body.radius.into(),
                    bevy::render::mesh::SphereKind::Ico { subdivisions: 15 },
                ))),
                MeshMaterial3d(material.add(body.color)),
                Transform::from_xyz(body.position.x, body.position.y, body.position.z),
            ));
        } else {
            handle.insert((
                Mesh3d(meshes.add(SphereMeshBuilder::new(
                    body.radius.into(),
                    bevy::render::mesh::SphereKind::Ico { subdivisions: 15 },
                ))),
                MeshMaterial3d(material.add(StandardMaterial {
                    base_color: body.color,
                    emissive: body.color.into(),
                    ..default()
                })),
                Transform::from_xyz(body.position.x, body.position.y, body.position.z),
                //GlobalTransform::default(),
            ));

            commands.entity(entity).with_child(PointLight {
                shadows_enabled: true,
                intensity: 30000000.0,
                range: 30.0,
                // radius: 10000000.0,
                color: body.color,
                ..default()
            });
        }
    }
}

// handle.insert((
//     Mesh3d(meshes.add(SphereMeshBuilder::new(
//         body.radius,
//         bevy::render::mesh::SphereKind::Ico { subdivisions: 15 },
//     ))),
//     MeshMaterial3d(material.add(body.color)),
//     Transform::from_xyz(body.position.x, body.position.y, body.position.z)
// ));

// if body.light {
//     handle.insert(PointLight {
//         shadows_enabled: true,
//         intensity: 100000000.0,
//         ..default()
//     });
// }
