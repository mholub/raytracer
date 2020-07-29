use crate::types::{Point3, Color};
use enum_dispatch::enum_dispatch;
use std::sync::Arc;

#[enum_dispatch]
#[derive(Clone)]
pub enum Texture {
    SolidColor,
    CheckerTexture
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
        CheckerTexture(Arc::new(Texture::SolidColor(SolidColor(color1))),
                       Arc::new(Texture::SolidColor(SolidColor(color2))))
    }
}

impl GetColor for CheckerTexture {
    fn value(&self, u: f32, v: f32, p: Point3) -> Color {
        let sines = (10.0 * p.x).sin() * (10.0 * p.y).sin() * (10.0 * p.z).sin();
        if sines < 0.0 { self.0.value(u, v, p) } else { self.1.value(u, v, p) }
    }
}