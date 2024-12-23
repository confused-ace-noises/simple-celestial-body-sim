use bevy::{color::Color, math::Vec3};
use clap::Parser;
use serde::Deserialize;

use crate::{cel_body::CelBody, sci_float::SciFloat};

#[derive(Debug, Clone, Deserialize)]
pub struct BodyBuilder {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [u8; 4],
    pub light: bool,
    pub velocity: [f32; 3],
    pub acceleration: [f32; 3],
    pub mass: f32,
} impl BodyBuilder {
    pub fn to_cel_body(self) -> CelBody {
        CelBody::new(Vec3::from_array(self.position), SciFloat::new(self.radius), Color::srgba_u8(self.color[0], self.color[1], self.color[2], self.color[3]), self.light, Vec3::new(self.velocity[0], self.velocity[1], self.velocity[2]), Vec3::new(self.acceleration[0], self.acceleration[1], self.acceleration[2]), SciFloat::new(self.mass))
    }
}


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub path: String
}
