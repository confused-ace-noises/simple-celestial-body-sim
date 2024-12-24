use bevy::prelude::Resource;
use cli::BodyBuilder;

pub mod camera;
pub mod cel_body;
pub mod sci_float;
pub mod cli;

pub const G: f32 = 6.67430e-11;

#[derive(Resource, Debug)]
pub struct JsonData(pub Vec<BodyBuilder>);