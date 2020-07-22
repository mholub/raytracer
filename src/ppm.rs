use std::io::Write;
use crate::types::Color;

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
        let ir = (255.999f32 * self.x) as u8;
        let ig = (255.999f32 * self.y) as u8;
        let ib = (255.999f32 * self.z) as u8;

        writeln!(writer, "{} {} {}", ir, ig, ib).unwrap();
    }
}