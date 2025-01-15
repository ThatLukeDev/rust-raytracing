struct Color {
    r: f64,
    g: f64,
    b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Self{
        Color { r, g, b }
    }

    pub fn bytes(&self) -> (f64, f64, f64) {
        ( self.r * 255.0, self.g * 255.0, self.b * 255.0 )
    }
}
