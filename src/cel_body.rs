use crate::JsonData;
use bevy::{prelude::*, render::mesh::SphereMeshBuilder};

#[derive(Component, Debug)]
pub struct CelBody;

#[derive(Component, Debug)]
pub struct Velocity(pub Vec3);

#[derive(Component, Debug)]
pub struct Acceleration(pub Vec3);

#[derive(Component, Debug)]
pub struct Mass(pub f32);

#[derive(Component, Debug)]
pub struct Radius(pub f32);

#[derive(Component, Debug)]
pub struct Color(pub bevy::color::Color);

#[derive(Component, Debug)]
pub struct Light(pub bool);

#[derive(Component, Debug)]
pub struct UpdatePosition;

#[derive(Component, Debug)]
pub struct NameTag(pub String);

#[derive(Debug, Bundle)]
pub struct CelBodyDataBundle {
    velocity: Velocity,
    acceleration: Acceleration,
    mass: Mass,
    radius: Radius,
    color: Color,
    light: Light,
}

#[derive(Debug, Bundle)]
pub struct CelBodyBundle<M: Material> {
    cel_body: CelBody,
    update_pos: UpdatePosition,
    cel_body_data: CelBodyDataBundle,
    mesh_3d: Mesh3d,
    mesh_material_3d: MeshMaterial3d<M>,
    transform: Transform,
}

pub fn init_cel_bodies(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<StandardMaterial>>,
    data: Res<JsonData>,
) {
    for builder in data.0.iter() {
        let color = bevy::prelude::Color::srgba_u8(
            builder.color[0],
            builder.color[1],
            builder.color[2],
            builder.color[3],
        );

        let entity = commands
            .spawn(CelBodyBundle {
                cel_body: CelBody,
                update_pos: UpdatePosition,
                cel_body_data: CelBodyDataBundle {
                    velocity: Velocity(Vec3::from_array(builder.velocity)),
                    acceleration: Acceleration(Vec3::from_array(builder.acceleration)),
                    mass: Mass(builder.mass),
                    radius: Radius(builder.radius),
                    color: Color(color),
                    light: Light(builder.light),
                },
                mesh_3d: Mesh3d(meshes.add(SphereMeshBuilder::new(
                    builder.radius.into(),
                    bevy::render::mesh::SphereKind::Ico { subdivisions: 15 },
                ))),
                mesh_material_3d: MeshMaterial3d(material.add(StandardMaterial {
                    base_color: color,
                    emissive: if builder.light {
                        color.into()
                    } else {
                        bevy::color::Color::srgba_u8(0, 0, 0, 0).into()
                    },
                    ..default()
                })),
                transform: Transform::from_xyz(
                    builder.position[0],
                    builder.position[1],
                    builder.position[2],
                ),
            })
            .id();

        if builder.light {
            commands.entity(entity).with_child(PointLight {
                shadows_enabled: true,
                intensity: 30000000.0,
                range: 30.0,
                color,
                ..default()
            });
        }
    }
}
