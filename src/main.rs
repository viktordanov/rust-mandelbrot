mod pixel;
mod ppm;
use std::fs;

use clap::Parser;
use pixel::Pixel;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value_t = 256)]
    width: u32,

    #[clap(short, long, value_parser, default_value_t = 256)]
    height: u32,

    #[clap(short, long, value_parser, default_value_t = 255)]
    max_iterations: u32,

    #[clap(short, long, value_parser, default_value = "out.ppm")]
    output: String,
}

fn main() {
    let args  = Args::parse();
    
    let width = args.width;
    let height = args.height;
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
    fs::write(args.output, ppm).unwrap();
}
