mod mandelbrot;
mod pixel;
mod png;

use clap::Parser;
use pixel::{Image, Persistable, Pixel};
use rayon::prelude::*;

#[derive(Parser)]
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

impl std::fmt::Debug for Args {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Args")
            .field("width", &self.width)
            .field("height", &self.height)
            .field("scale", &self.scale)
            .field("max_iterations", &self.max_iterations)
            .field("output", &self.output)
            .field("zoom", &self.zoom)
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}

fn main() {
    let args = Args::parse();

    write_frame(&args, 0);
}

fn write_frame(args: &Args, frame_i: usize) {
    let width: f64 = (args.width * args.scale as u32).into();
    let height: f64 = (args.height * args.scale as u32).into();
    let mut image: Image = vec![Pixel::new(0, 0, 0); (width * height) as usize];

    let chunk_size = (width / 16_f64) as usize;
    image
        .par_chunks_mut(chunk_size)
        .enumerate()
        .for_each(|(chunk_index, chunk)| {
            let start = chunk_index * chunk_size;
            let end = start + chunk_size;

            let mut i = 0;
            for pixel_i in start..end {
                // calculate the pixel's x and y coordinates with zoom and pan
                let mut x = (pixel_i % width as usize) as f64 / width * 3.5;
                let mut y = (pixel_i / width as usize) as f64 / height * 2.0;

                // center coord
                x -= 3.5 / 2.0;
                y -= 2.0 / 2.0;

                // zoom
                x /= args.zoom;
                y /= args.zoom;

                // undo center
                x += 3.5 / 2.0;
                y += 2.0 / 2.0;

                // pan
                x += args.x - 2.5;
                y += args.y - 1.0;

                let (r, g, b) = mandelbrot::get_color_at(x, y, args.max_iterations);

                let pixel = Pixel::new(r as u8, g as u8, b as u8);
                chunk[i] = pixel;
                i += 1;
            }
        });

    let path = format!("frame_{}.png", frame_i);
    match image.save(&path, width as u32, height as u32) {
        Ok(_) => println!("Saved frame {} to {}", frame_i, path),
        Err(e) => println!("Error saving frame {}: {}", frame_i, e),
    };
}
