use num::Complex;
use palette::{Srgb, FromColor, Hsv};

fn min<T: PartialOrd>(a: T, b: T) -> T {
	if a < b {
		a
	} else {
		b
	}
}

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
		let V = (i as f64).ln();

		// (255,255,255) * (1 + cos(2 * pi * (V - 0.5))) / 2

		let hsv = Hsv::new(360.0 * (1.0 + (V).cos()) / 2.0, 1.0, 1.0);
		let rgb = Srgb::from_hsv(hsv);
		let r = (rgb.red * 255.0) as u8;
		let g = (rgb.green * 255.0) as u8;
		let b = (rgb.blue * 255.0) as u8;
		(r, g, b)
	



		// let smooth = i as f64 + 1_f64 - (z.norm().ln().log2() / 2.0_f64.ln());
			  
		// let hsb = (smooth * 6.0 as f64, 0.7, 0.1 + min(smooth*smooth/max_iterations as f64, 0.86)); 
		// let hsb = Hsv::new(hsb.0, hsb.1, hsb.2); 
		// let srgb = Srgb::from_hsv(hsb);
		  
		//   (
		// 	  (srgb.red * 255.0) as u8,
		// 	  (srgb.green * 255.0) as u8,
		// 	  (srgb.blue * 255.0) as u8,
		//   )
    }
}
