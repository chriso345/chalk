use std::cell::RefCell;

use wasm_bindgen::JsCast;
use web_sys::{
    CanvasRenderingContext2d, HtmlCanvasElement, OffscreenCanvas, OffscreenCanvasRenderingContext2d,
};

use crate::canvas::state::WhiteboardState;

pub mod dots;
pub mod grids;

pub const BASE_SPACING: f64 = 32.0;
pub const MIN_SCREEN_SPACING: f64 = 24.0;
pub const MIN_ALPHA: f64 = 0.01;

const CACHE_GRID: f64 = 2.0;

#[derive(PartialEq, Clone)]
pub struct BackgroundCacheKey {
    pub spacing_q: u32,
    pub phase_x_q: u32,
    pub phase_y_q: u32,
    pub width: u32,
    pub height: u32,
    pub dark_mode: bool,
    pub variant: u8,
}

pub struct BackgroundCache {
    pub key: BackgroundCacheKey,
    pub offscreen: OffscreenCanvas,
}

thread_local! {
    pub static BG_CACHE: RefCell<Option<BackgroundCache>> = RefCell::new(None);
}

pub fn get_screen_spacing(zoom: f64) -> f64 {
    (BASE_SPACING * zoom).max(MIN_SCREEN_SPACING)
}

pub fn with_cache<F>(key: BackgroundCacheKey, w: u32, h: u32, draw_fn: F) -> OffscreenCanvas
where
    F: FnOnce(&OffscreenCanvasRenderingContext2d),
{
    BG_CACHE.with(|cell| {
        let mut cache = cell.borrow_mut();

        if let Some(ref c) = *cache {
            if c.key == key {
                return c.offscreen.clone();
            }
        }

        let oc = OffscreenCanvas::new(w, h).unwrap();
        let ctx = oc
            .get_context("2d")
            .unwrap()
            .unwrap()
            .dyn_into::<OffscreenCanvasRenderingContext2d>()
            .unwrap();

        draw_fn(&ctx);

        let out = oc.clone();
        *cache = Some(BackgroundCache { key, offscreen: oc });
        out
    })
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum BackgroundKind {
    None,
    Dots,
    Grid,
}

impl BackgroundKind {
    pub fn draw(
        self,
        ctx: &CanvasRenderingContext2d,
        canvas: &HtmlCanvasElement,
        state: &WhiteboardState,
    ) {
        match self {
            BackgroundKind::Dots => dots::draw(ctx, canvas, state),
            BackgroundKind::Grid => grids::draw(ctx, canvas, state),
            BackgroundKind::None => {}
        }
    }

    pub fn next(self) -> Self {
        match self {
            BackgroundKind::None => BackgroundKind::Dots,
            BackgroundKind::Dots => BackgroundKind::Grid,
            BackgroundKind::Grid => BackgroundKind::None,
        }
    }

    pub fn from_name(name: &str) -> Self {
        match name {
            "dots" => BackgroundKind::Dots,
            "grid" => BackgroundKind::Grid,
            "none" => BackgroundKind::None,
            _ => panic!("Unknown background kind: {}", name),
        }
    }
}
