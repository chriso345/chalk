pub struct ColorEntry {
    pub name: &'static str,
    pub color: ChalkColor,
}

pub const COLORS: &[ColorEntry] = &[
    ColorEntry {
        name: "white",
        color: ChalkColor::WHITE,
    },
    ColorEntry {
        name: "black",
        color: ChalkColor::BLACK,
    },
    ColorEntry {
        name: "red",
        color: ChalkColor::RED,
    },
    ColorEntry {
        name: "green",
        color: ChalkColor::GREEN,
    },
    ColorEntry {
        name: "blue",
        color: ChalkColor::BLUE,
    },
    ColorEntry {
        name: "orange",
        color: ChalkColor::ORANGE,
    },
    ColorEntry {
        name: "purple",
        color: ChalkColor::PURPLE,
    },
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ChalkColor {
    hex: [u8; 7],
}

impl ChalkColor {
    pub const BLACK: Self = Self { hex: *b"#000000" };
    pub const WHITE: Self = Self { hex: *b"#FFFFFF" };
    pub const RED: Self = Self { hex: *b"#E24B4A" };
    pub const GREEN: Self = Self { hex: *b"#4AB557" };
    pub const BLUE: Self = Self { hex: *b"#378ADD" };
    pub const ORANGE: Self = Self { hex: *b"#E27B4A" };
    pub const PURPLE: Self = Self { hex: *b"#A24BD4" };

    pub fn new() -> Self {
        Self::WHITE
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        const LUT: &[u8; 16] = b"0123456789ABCDEF";

        Self {
            hex: [
                b'#',
                LUT[(r >> 4) as usize],
                LUT[(r & 0xF) as usize],
                LUT[(g >> 4) as usize],
                LUT[(g & 0xF) as usize],
                LUT[(b >> 4) as usize],
                LUT[(b & 0xF) as usize],
            ],
        }
    }

    pub fn from_hex(hex: &str) -> Option<Self> {
        let hex = hex.trim_start_matches('#');

        if hex.len() != 6 || !hex.chars().all(|c| c.is_ascii_hexdigit()) {
            return None;
        }

        let mut out = [0u8; 7];
        out[0] = b'#';

        for i in 0..6 {
            out[i + 1] = hex.as_bytes()[i].to_ascii_uppercase();
        }

        Some(Self { hex: out })
    }

    pub fn from_word(word: &str) -> Option<Self> {
        COLORS.iter().find(|e| e.name == word).map(|e| e.color)
    }

    pub fn to_word(&self) -> Option<&'static str> {
        COLORS.iter().find(|e| e.color == *self).map(|e| e.name)
    }

    pub fn to_hex(&self) -> &str {
        std::str::from_utf8(&self.hex).unwrap()
    }

    pub fn r(&self) -> u8 {
        u8::from_str_radix(&self.to_hex()[1..3], 16).unwrap()
    }

    pub fn g(&self) -> u8 {
        u8::from_str_radix(&self.to_hex()[3..5], 16).unwrap()
    }

    pub fn b(&self) -> u8 {
        u8::from_str_radix(&self.to_hex()[5..7], 16).unwrap()
    }
}
