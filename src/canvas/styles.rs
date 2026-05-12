pub struct ChalkStyles<'a> {
    /// The background color of the chalkboard
    pub canvas_color: &'a str,
}

impl ChalkStyles<'_> {
    pub fn light() -> Self {
        Self {
            canvas_color: "#1A1A18",
        }
    }
    pub fn dark() -> Self {
        Self {
            canvas_color: "#F2F0EF",
        }
    }

    pub fn get_bg(&self) -> &str {
        self.canvas_color
    }
}
