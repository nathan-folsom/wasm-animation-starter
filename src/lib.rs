use crate::constants::*;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod constants;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub struct Renderer {
    frame_count: i32,
}

#[wasm_bindgen]
impl Renderer {
    #[wasm_bindgen(constructor)]
    pub fn js_constructor() -> Self {
        Self { frame_count: 0 }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, ctx: &CanvasRenderingContext2d) {}

    #[wasm_bindgen]
    pub fn render_frame(&mut self, ctx: &CanvasRenderingContext2d) -> i32 {
        self.frame_count += 1;
        self.frame_count
    }

    #[wasm_bindgen]
    pub fn render_overlay(&mut self, ctx: &CanvasRenderingContext2d) {}
}
