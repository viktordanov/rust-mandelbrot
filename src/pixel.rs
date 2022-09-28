#[derive(Clone, Copy, Debug, Default)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn set(&mut self, r: u8, g: u8, b: u8) {
        self.r = r;
        self.g = g;
        self.b = b;
    }

    pub fn make_brighter(&mut self, amount: u8) {
        self.r = self.r.saturating_add(amount);
        self.g = self.g.saturating_add(amount);
        self.b = self.b.saturating_add(amount);
    }
}

pub type Image = Vec<Pixel>;

pub trait Persistable {
    fn save(&self, path: &str, width: u32, height: u32) -> Result<(), std::io::Error>;
}
 

pub fn image_to_byte_array(image: &Image) -> Vec<u8> {
    let mut bytes = Vec::new();
    for pixel in image {
        bytes.push(pixel.r);
        bytes.push(pixel.g);
        bytes.push(pixel.b);
        bytes.push(0xFF);
    }
    bytes
}
