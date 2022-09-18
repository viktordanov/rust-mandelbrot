mod pixel;
mod ppm;
use std::fs;

use pixel::Pixel;


fn main() {
    let width = 256;
    let height = 256;
    let mut pixels = vec![Pixel::new(0, 0, 0); (width * height) as usize];
    let mut i = 0;

    for y in 0..height {
        for x in 0..width {
            let pixel = &mut pixels[i];
            let r = x as u8;
            let g = y as u8;
            let b = 0;
            pixel.set(r, g, b);
            i += 1;
        }
    }

    let ppm = ppm::write_pixels_to_ppm_string(&pixels, width, height);
    fs::write("out.ppm", ppm).unwrap();
}
