use num::Complex;

pub fn get_color_at(x: f64, y: f64, max_iterations: u32) -> (u8, u8, u8) {
    let mut z = Complex::new(0.0, 0.0);
    let c = Complex::new(x, y);
    let mut i = 0;
    while i < max_iterations && z.norm() < 2.0 {
        z = z * z + c;
        i += 1;
    }
    if i == max_iterations {
        (0, 0, 0)
    } else {
        let color = (i as f64 / max_iterations as f64) * 255.0;
        (color as u8, color as u8, color as u8)
    }
}
