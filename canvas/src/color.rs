pub type Rgba = (f32, f32, f32, f32);

pub trait Color {
    fn normalize(self) -> Rgba;
}

/// Integer RGBA
impl Color for (u8, u8, u8, f32) {
    fn normalize(self) -> Rgba {
        (self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0, self.3)
    }
}

/// Integer RGB
impl Color for (u8, u8, u8) {
    fn normalize(self) -> Rgba {
        (self.0 as f32 / 255.0, self.1 as f32 / 255.0, self.2 as f32 / 255.0, 1.0)
    }
}

/// Fp RGBA
impl Color for (f32, f32, f32, f32) {
    fn normalize(self) -> Rgba {
        self
    }
}

/// Fp RGB
impl Color for (f32, f32, f32) {
    fn normalize(self) -> Rgba {
        (self.0, self.1, self.2, 1.0)
    }
}
