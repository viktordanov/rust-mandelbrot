mod mandelbrot;
mod pixel;
mod ppm;
use std::fs;

use clap::Parser;
use pixel::Pixel;
use rayon::prelude::*;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long, value_parser, default_value_t = 256)]
    width: u32,

    #[clap(short, long, value_parser, default_value_t = 256)]
    height: u32,

    #[clap(short, long, value_parser, default_value_t = 1.0)]
    scale: f32,

    #[clap(short, long, value_parser, default_value_t = 255)]
    max_iterations: u32,

    #[clap(short, long, value_parser, default_value = "out.ppm")]
    output: String,

    // zoom
    #[clap(short, long, value_parser, default_value_t = 1.0)]
    zoom: f64,

    // pan
    #[clap(short, long, value_parser, default_value_t = 0.0)]
    x: f64,

    #[clap(short, long, value_parser, default_value_t = 0.0)]
    y: f64,
}

fn main() {
    let args = Args::parse();

    let width = args.width * args.scale as u32;
    let height = args.height * args.scale as u32;
    let mut pixels = vec![Pixel::new(0, 0, 0); (width * height) as usize];

    let chunk_size = (width / 16) as usize;
    pixels
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            let start = chunk_index * chunk_size;
            let end = start + chunk_size;

            let mut i = 0;
            for pixel_i in start..end {
                // calculate the pixel's x and y coordinates with zoom and pan
                let x = (pixel_i % width as usize) as f64 / width as f64 * 3.5 - 2.5 + args.x;
                let y = (pixel_i / width as usize) as f64 / height as f64 * 2.0 - 1.0 + args.y;

                // zoom in center
                let x = x / args.zoom;
                let y = y / args.zoom;

                let (r, g, b) = mandelbrot::get_color_at(x as f64, y as f64, args.max_iterations);
                let pixel = Pixel::new(r, g, b);
                chunk[i] = pixel;
                i += 1;
            }
        });

    let ppm = ppm::write_pixels_to_ppm_string(&pixels, width, height);
    fs::write(args.output, ppm).unwrap();
}
