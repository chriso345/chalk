#[derive(Clone, Debug, Copy)]
pub struct ChalkColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ChalkColor {
    pub fn new() -> Self {
        // Black
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
            let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
            let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
            Some(Self { r, g, b })
        } else {
            None
        }
    }

    pub fn to_hex(&self) -> String {
        format!("#{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }

    pub fn from_word(word: &str) -> Option<Self> {
        match word.to_lowercase().as_str() {
            "black" => Some(Self::from_rgb(0, 0, 0)),
            "white" => Some(Self::from_rgb(255, 255, 255)),
            "red" => Some(Self::from_rgb(226, 75, 74)),
            "green" => Some(Self::from_rgb(74, 181, 87)),
            "blue" => Some(Self::from_rgb(55, 138, 221)),
            _ => None,
        }
    }
}
