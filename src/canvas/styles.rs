pub struct ChalkStyles<'a> {
    /// The background color of the chalkboard
    pub canvas_color: &'a str,

    pub stroke_1: &'a str,
    // Unused for now
    pub stroke_2: &'a str,
}

impl ChalkStyles<'_> {
    pub fn light() -> Self {
        Self {
            canvas_color: "#1A1A18",

            stroke_1: "#F2F0EF",
            stroke_2: "#F2F0EF",
        }
    }
    pub fn dark() -> Self {
        Self {
            canvas_color: "#F2F0EF",

            stroke_1: "#1A1A18",
            stroke_2: "#1A1A18",
        }
    }

    pub fn get_bg(&self) -> &str {
        self.canvas_color
    }

    pub fn get_stroke(&self) -> &str {
        self.stroke_1
    }
}
