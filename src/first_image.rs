use std::io::Write;
use crate::types::Color;
use crate::ppm::*;

fn get_color(u:f32, v:f32) -> Color {
    let r = u;
    let g = v;
    let b = 0.25;
    Color::new(r,g,b)
}

pub fn main() {
    let image_width = 256;
    let image_height = 256;

    write_header(&mut std::io::stdout(), image_width, image_height);
    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        std::io::stderr().flush().unwrap();
        for i in 0..image_width {
            let u = (i as f32) / (image_width - 1) as f32;
            let v = (j as f32) / (image_height - 1) as f32;

            get_color(u, v).write_ppm(&mut std::io::stdout());
        }
    }
    eprint!("\nDone\n");
}