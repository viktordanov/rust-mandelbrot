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
