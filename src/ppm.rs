use std::io::Write;
use crate::types::Color;
use nalgebra_glm::{clamp_scalar};

pub fn write_header<W:Write>(writer : &mut W, width : i32, height : i32) {
    writeln!(writer, "P3").unwrap();
    writeln!(writer, "{} {}", width, height).unwrap();
    writeln!(writer, "255").unwrap();
}

pub trait WritePPM {
    fn write_ppm<W:Write>(&self, writer : &mut W);
}

impl WritePPM for Color {
    fn write_ppm<W:Write>(&self, writer : &mut W) {
        let ir = (256.0 * clamp_scalar(self.x, 0.0, 0.999)) as u8;
        let ig = (256.0 * clamp_scalar(self.y, 0.0, 0.999)) as u8;
        let ib = (256.0 * clamp_scalar(self.z, 0.0, 0.999)) as u8;

        writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
    }
}