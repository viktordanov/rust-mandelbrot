use num::Complex;
use palette::{Srgb, FromColor, Hsv};


pub fn get_color_at(x: f64, y: f64, max_iterations: u32) -> (u8, u8, u8) {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);
    let mut i = 0;
    while i < max_iterations && z.norm() < 200.0 {
        z = z * z + c;
        i += 1;
    }
    if i == max_iterations {
        (0, 0, 0)
    } else {
		let v = (i as f64).ln();

		let hsv = Hsv::new(360.0 * (1.0 + (v).cos()) / 2.0, 1.0, 1.0);
		let rgb = Srgb::from_hsv(hsv);
		let r = (rgb.red * 255.0) as u8;
		let g = (rgb.green * 255.0) as u8;
		let b = (rgb.blue * 255.0) as u8;
		(r, g, b)
    }
}
