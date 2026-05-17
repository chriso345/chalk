use crate::canvas::background::BackgroundKind;

pub struct ChalkStyles<'a> {
    /// The background color of the chalkboard
    pub canvas_color: &'a str,

    pub canvas_bg_color: &'a str,
    pub canvas_bg: BackgroundKind,

    pub is_dark: bool,
}

impl ChalkStyles<'_> {
    pub fn light() -> Self {
        Self {
            canvas_color: "#1A1A18",
            canvas_bg_color: "rgba(242,240,239,0.25)",
            canvas_bg: BackgroundKind::None,
            is_dark: false,
        }
    }
    pub fn dark() -> Self {
        Self {
            canvas_color: "#F2F0EF",
            canvas_bg_color: "rgba(26,26,24,0.20)",
            canvas_bg: BackgroundKind::None,
            is_dark: true,
        }
    }

    pub fn toggle(&mut self) {
        let mut new = if self.is_dark {
            Self::light()
        } else {
            Self::dark()
        };

        new.canvas_bg = self.canvas_bg;
        *self = new;
    }

    pub fn get_bg(&self) -> &str {
        self.canvas_color
    }

    pub fn is_dark(&self) -> bool {
        self.is_dark
    }

    pub fn is_light(&self) -> bool {
        !self.is_dark
    }

    pub fn get_canvas_bg_color(&self) -> &str {
        self.canvas_bg_color
    }
}
