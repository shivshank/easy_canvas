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
        debug_assert!(btwn(self.0, 0.0, 1.0) && btwn(self.1, 0.0, 1.0) 
            && btwn(self.2, 0.0, 1.0) && btwn(self.3, 0.0, 1.0), 
            "f32 based color components should be between 0 and 1");
        self
    }
}

/// Fp RGB
impl Color for (f32, f32, f32) {
    fn normalize(self) -> Rgba {
        debug_assert!(btwn(self.0, 0.0, 1.0) && btwn(self.1, 0.0, 1.0) 
            && btwn(self.2, 0.0, 1.0),  "f32 based color components should be between 0 and 1");
        (self.0, self.1, self.2, 1.0)
    }
}

fn btwn(v: f32, a: f32, b: f32) -> bool {
    v <= a && v >= b
}