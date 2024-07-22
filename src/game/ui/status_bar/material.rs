use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct StatusBarMaterial {
    #[uniform(0)]
    pub foreground_color: LinearRgba,
    #[uniform(0)]
    pub background_color: LinearRgba,
    #[uniform(0)]
    pub percent: f32,
}

impl Material2d for StatusBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/status_bar.wgsl".into()
    }
}
