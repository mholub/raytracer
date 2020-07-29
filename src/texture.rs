use crate::types::{Point3, Color};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;
use crate::perlin::Perlin;

#[enum_dispatch]
#[derive(Clone)]
pub enum Texture {
    SolidColor,
    CheckerTexture,
    NoiseTexture
}

#[enum_dispatch(Texture)]
pub trait GetColor {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color;
}

#[derive(Copy, Clone)]
pub struct SolidColor(pub Color);

impl GetColor for SolidColor {
    fn value(&self, _u: f32, _v: f32, _p: Point3) -> Color {
        self.0
    }
}

#[derive(Clone)]
pub struct CheckerTexture(pub Arc<dyn GetColor + Send + Sync>, pub Arc<dyn GetColor + Send + Sync>);

impl CheckerTexture {
    pub fn from_colors(color1: Color, color2: Color) -> Self {
        CheckerTexture(Arc::new(Texture::from(SolidColor(color1))),
                       Arc::new(Texture::from(SolidColor(color2))))
    }
}

impl GetColor for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 { self.0.value(u, v, p) } else { self.1.value(u, v, p) }
    }
}

#[derive(Clone)]
pub struct NoiseTexture(f32, Perlin);

impl NoiseTexture {
    pub fn new(scale: f32) -> Self {
        Self(scale, Perlin::new())
    }
}

impl GetColor for NoiseTexture {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color {
        Color::new(1.0, 1.0, 1.0) * 0.5 * (1.0 + (self.0*p.z + 10.0 * self.1.turb(p, 7)).sin())
    }
}