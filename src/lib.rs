// NOTE: Very temp, think about it better later
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rgb {
    r: f32,
    g: f32,
    b: f32,
}

impl Default for Rgb {
    fn default() -> Self {
        Self {
            r: 1.,
            g: 1.,
            b: 1.,
        }
    }
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

    pub fn to_array(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }

    pub fn to_rgba_array(self) -> [f32; 4] {
        [self.r, self.g, self.b, 1.0]
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct QuadColors {
    top_left: Rgb,
    top_right: Rgb,
    bottom_left: Rgb,
    bottom_right: Rgb,
}

impl QuadColors {
    pub fn new(top_left: Rgb, top_right: Rgb, bottom_left: Rgb, bottom_right: Rgb) -> Self {
        Self {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn new_from_rgb(rgb: Rgb) -> Self {
        Self {
            top_left: rgb,
            top_right: rgb,
            bottom_left: rgb,
            bottom_right: rgb,
        }
    }

    pub fn new_top_bottom(top: Rgb, bottom: Rgb) -> Self {
        Self {
            top_left: top,
            top_right: top,
            bottom_left: bottom,
            bottom_right: bottom,
        }
    }

    pub fn top_left(&self) -> Rgb {
        self.top_left
    }

    pub fn top_right(&self) -> Rgb {
        self.top_right
    }

    pub fn bottom_left(&self) -> Rgb {
        self.bottom_left
    }

    pub fn bottom_right(&self) -> Rgb {
        self.bottom_right
    }
}
