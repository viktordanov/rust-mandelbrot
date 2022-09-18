use crate::pixel::Pixel;

pub fn write_pixels_to_ppm_string(pixels: &[Pixel], width: u32, height: u32) -> String {
	let mut ppm = String::new();
	ppm.push_str("P3\n");
	ppm.push_str(&format!("{} {}\n", width, height));
	ppm.push_str("255\n");

	let mut i = 0;
	for pixel in pixels {
		if i % width == 0 && i != 0 {
			ppm.push_str("\n");
		}
		ppm.push_str(&format!("{} {} {} ", pixel.r, pixel.g, pixel.b));
		i+=1;
	}

	ppm
}