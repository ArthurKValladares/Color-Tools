// NOTE: Very temp, think about it better later
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Rgb {
    pub fn new_from_u8(red: u8, green: u8, blue: u8) -> Self {
        Self {
            r: red as f32 / 255.,
            g: green as f32 / 255.,
            b: blue as f32 / 255.,
        }
    }

    pub fn red(&self) -> f32 {
        self.r
    }

    pub fn green(&self) -> f32 {
        self.g
    }

    pub fn blue(&self) -> f32 {
        self.b
    }
}
