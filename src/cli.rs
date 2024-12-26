use clap::Parser;
use serde::Deserialize;


#[derive(Debug, Clone, Deserialize)]
pub struct BodyBuilder {
    pub position: [f32; 3],
    pub radius: f32,
    pub color: [u8; 4],
    pub light: bool,
    pub velocity: [f32; 3],
    pub acceleration: [f32; 3],
    pub mass: f32,
}


#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    pub path: String
}
