use core::fmt;
use std::sync::Arc;

use crate::ui::overlay::OverlayContext;

/// Which screen edge (or corner) a panel is pinned to.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Anchor {
    TopLeft,
    TopCenter,
    TopRight,
    BottomLeft,
    BottomCenter,
    BottomRight,
    CenterLeft,
    CenterRight,
    Center,
}

/// Direction children are stacked inside a panel.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Row,
    Column,
}

/// Label types can be either text or listeners for signals (e.g. zoom level).
#[derive(Clone)]
pub enum Label {
    Static(&'static str),
    Dynamic(Arc<dyn Fn(&OverlayContext) -> String + Send + Sync>),
}

impl fmt::Debug for Label {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Label::Static(s) => f.debug_tuple("Static").field(s).finish(),
            Label::Dynamic(_) => f.write_str("Signal(<fn>)"),
        }
    }
}

/// A single UI element inside a panel.
#[derive(Clone, Debug)]
pub struct BoxConfig {
    /// Unique key - used as the HTML element key and for event dispatch.
    pub id: &'static str,
    /// Optional display label (used by Button, Badge, etc.).
    pub label: Option<Label>,
    /// Optional Shortcut hint
    pub hint: Option<&'static str>,
    /// The kind of box to render.
    pub kind: BoxKind,
    /// Children are only used for groups - they are ignored for buttons, badges, etc.
    pub children: Option<Vec<BoxConfig>>,
}

/// Describes what type of component a `BoxConfig` maps to.
#[derive(Clone, Debug)]
pub enum BoxKind {
    /// Clickable button. `on_click` is the action id forwarded to the app.
    Button {
        action: &'static str,
    },
    /// Static text label.
    Label,
    /// Horizontal or vertical divider.
    Divider,
    /// Image
    Image {
        src: &'static str,
    },
    // Circle filled with `color`, acts as a button.
    Swatch {
        color: &'static str,
        action: &'static str,
    },
    // Horizontal line at `width` thickness, acts as a button.
    StrokeWidth {
        width: u32,
        action: &'static str,
    },
}

/// A panel groups related boxes and pins them to a screen anchor.
#[derive(Clone, Debug)]
pub struct PanelConfig {
    /// Unique panel id.
    pub id: &'static str,
    /// Where on screen this panel is placed.
    pub anchor: Anchor,
    /// Pixel offset from the anchor point (dx, dy).
    pub offset: (i32, i32),
    /// How children are arranged.
    pub direction: Direction,
    /// Gap between children in pixels.
    pub gap: u32,
    /// Padding inside the panel in pixels.
    pub padding: u32,
    /// The boxes that live inside this panel.
    pub children: Vec<BoxConfig>,
}

impl PanelConfig {
    pub fn new(id: &'static str, anchor: Anchor) -> Self {
        Self {
            id,
            anchor,
            offset: (0, 0),
            direction: Direction::Row,
            gap: 8,
            padding: 8,
            children: vec![],
        }
    }

    pub fn offset(mut self, dx: i32, dy: i32) -> Self {
        self.offset = (dx, dy);
        self
    }

    pub fn direction(mut self, dir: Direction) -> Self {
        self.direction = dir;
        self
    }

    pub fn gap(mut self, px: u32) -> Self {
        self.gap = px;
        self
    }

    pub fn padding(mut self, px: u32) -> Self {
        self.padding = px;
        self
    }

    pub fn add(mut self, child: BoxConfig) -> Self {
        self.children.push(child);
        self
    }
}

impl BoxConfig {
    pub fn button(id: &'static str, label: Label, action: &'static str) -> Self {
        Self {
            id,
            label: Some(label),
            hint: None,
            kind: BoxKind::Button { action },
            children: None,
        }
    }

    pub fn icon_button(
        id: &'static str,
        icon_id: &'static str,
        src: &'static str,
        action: &'static str,
    ) -> Self {
        Self {
            id,
            label: None,
            hint: None,
            kind: BoxKind::Button { action },
            children: Some(vec![BoxConfig {
                id: icon_id,
                label: None,
                hint: None,
                kind: BoxKind::Image { src },
                children: None,
            }]),
        }
    }

    pub fn label(id: &'static str, text: Label) -> Self {
        Self {
            id,
            label: Some(text),
            hint: None,
            kind: BoxKind::Label,
            children: None,
        }
    }

    pub fn divider() -> Self {
        Self {
            id: "divider",
            label: None,
            hint: None,
            kind: BoxKind::Divider,
            children: None,
        }
    }

    pub fn swatch(id: &'static str, color: &'static str, action: &'static str) -> Self {
        Self {
            id,
            label: None,
            hint: None,
            kind: BoxKind::Swatch { color, action },
            children: None,
        }
    }

    pub fn stroke_width(id: &'static str, width: u32, action: &'static str) -> Self {
        Self {
            id,
            label: None,
            hint: None,
            kind: BoxKind::StrokeWidth { width, action },
            children: None,
        }
    }

    pub fn with_hint(&mut self, hint: &'static str) -> Self {
        self.hint = Some(hint);
        self.clone()
    }
}

impl Anchor {
    /// Returns the `position:fixed` CSS snippet that places the panel.
    pub fn to_css(&self, offset: (i32, i32)) -> String {
        let (dx, dy) = offset;
        match self {
            Anchor::TopLeft => format!("top:{}px;left:{}px;", dy, dx),
            Anchor::TopCenter => format!(
                "top:{}px;left:50%;transform:translateX(-50%) translateX({}px);",
                dy, dx
            ),
            Anchor::TopRight => format!("top:{}px;right:{}px;", dy, -dx),
            Anchor::BottomLeft => format!("bottom:{}px;left:{}px;", -dy, dx),
            Anchor::BottomCenter => format!(
                "bottom:{}px;left:50%;transform:translateX(-50%) translateX({}px);",
                -dy, dx
            ),
            Anchor::BottomRight => format!("bottom:{}px;right:{}px;", -dy, -dx),
            Anchor::CenterLeft => format!(
                "top:50%;left:{}px;transform:translateY(-50%) translateY({}px);",
                dx, dy
            ),
            Anchor::CenterRight => format!(
                "top:50%;right:{}px;transform:translateY(-50%) translateY({}px);",
                -dx, dy
            ),
            Anchor::Center => format!(
                "top:50%;left:50%;transform:translate(-50%,-50%) translate({}px,{}px);",
                dx, dy
            ),
        }
    }
}

impl Direction {
    pub fn to_flex_css(&self) -> &'static str {
        match self {
            Direction::Row => "row",
            Direction::Column => "column",
        }
    }
}
