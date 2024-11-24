use crate::constants::*;
use crate::math::*;
use std::collections::VecDeque;
use std::f64::consts::PI;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

mod constants;
mod math;

// When the `wee_alloc` feature is enabled, this uses `wee_alloc` as the global
// allocator.
//
// If you don't want to use `wee_alloc`, you can safely delete this.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    // This provides better error messages in debug mode.
    // It's disabled in release mode so it doesn't bloat up the file size.
    #[cfg(debug_assertions)]
    console_error_panic_hook::set_once();

    Ok(())
}

#[wasm_bindgen]
pub struct FieldRenderer {
    rotor_renderers: Vec<RotorRenderer>,
    frame_count: i32,
}

#[wasm_bindgen]
impl FieldRenderer {
    #[wasm_bindgen(constructor)]
    pub fn ctr() -> Self {
        Self {
            rotor_renderers: vec![
                RotorRenderer::new(
                    Rotor {
                        r: 160.0,
                        theta: 0.0,
                        v: 0.0001004,
                        l: 1080.0,
                        c_r: 480.0,
                        c_theta: -SPIN_OFFSET,
                        c_v: 0.00004,
                        origin_offset: (0.0, 0.0),
                    },
                    Rotor {
                        r: 160.0,
                        theta: PI / 4.0,
                        v: 0.0001,
                        l: 1200.0,
                        c_r: 480.0,
                        c_theta: PI / 3.1 - SPIN_OFFSET,
                        c_v: 0.00004,
                        origin_offset: (0.0, 0.0),
                    },
                    String::from("#ff00004f"),
                ),
                RotorRenderer::new(
                    Rotor {
                        r: 160.0,
                        theta: 0.0,
                        v: 0.00001004,
                        l: 1080.0,
                        c_r: 280.0,
                        c_theta: 0.0,
                        c_v: 0.00006,
                        origin_offset: (0.0, 0.0),
                    },
                    Rotor {
                        r: 160.0,
                        theta: PI / 4.0,
                        v: 0.00001,
                        l: 1200.0,
                        c_r: 200.0,
                        c_theta: PI / 3.1,
                        c_v: 0.00006,
                        origin_offset: (0.0, 0.0),
                    },
                    String::from("#ffffff"),
                ),
            ],
            frame_count: 0,
        }
    }

    #[wasm_bindgen]
    pub fn init(&mut self, ctx: &CanvasRenderingContext2d) {
        self.render_background(ctx, true);
        self.rotor_renderers.iter_mut().for_each(|r| {
            r.compute_points();
        });
    }

    #[wasm_bindgen]
    pub fn render_frame(&mut self, ctx: &CanvasRenderingContext2d) -> i32 {
        self.frame_count += 1;
        self.render_background(ctx, false);
        self.rotor_renderers.iter_mut().for_each(|r| {
            r.render_line(ctx);
        });
        self.frame_count
    }

    fn render_background(&self, ctx: &CanvasRenderingContext2d, opaque: bool) {
        if opaque {
            ctx.set_fill_style(&JsValue::from_str("#03000f"));
        } else {
            ctx.set_fill_style(&JsValue::from_str("#03000f01"));
        }
        ctx.fill_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);
        ctx.set_stroke_style(&JsValue::from("#ffffff"));
    }

    #[wasm_bindgen]
    pub fn render_overlay(&mut self, ctx: &CanvasRenderingContext2d) {
        self.rotor_renderers.iter_mut().for_each(|r| {
            r.render_overlay(ctx);
        });
    }
}

struct RotorRenderer {
    rotors: (Rotor, Rotor),
    points: VecDeque<(f64, f64)>,
    color: String,
}

impl RotorRenderer {
    pub fn new(a: Rotor, b: Rotor, color: String) -> Self {
        Self {
            rotors: (a, b),
            points: VecDeque::new(),
            color,
        }
    }

    pub fn render_overlay(&self, ctx: &CanvasRenderingContext2d) {
        ctx.clear_rect(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT);

        ctx.set_stroke_style(&JsValue::from_str("#ff000055"));
        ctx.set_line_width(5.0);
        ctx.set_fill_style(&JsValue::from_str("#000000"));
        let render_rim = |rotor: &Rotor| {
            ctx.begin_path();
            let (cx, cy) = rotor.get_center();
            let _ = ctx.ellipse(cx, cy, rotor.r, rotor.r, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };
        render_rim(&self.rotors.0);
        render_rim(&self.rotors.1);

        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        let points = (self.rotors.0.get_point(), self.rotors.1.get_point());
        let render_point = |point: (f64, f64)| {
            let (x, y) = point;
            ctx.begin_path();
            let _ = ctx.ellipse(x, y, 2.0, 2.0, 0.0, 0.0, 2.0 * PI);
            ctx.stroke();
        };
        render_point(points.0);
        render_point(points.1);

        ctx.set_stroke_style(&JsValue::from_str("#ff000055"));
        let intersections = get_intersection(&self.rotors.0, &self.rotors.1);
        ctx.begin_path();
        ctx.move_to(points.0 .0, points.0 .1);
        ctx.line_to(intersections.1 .0, intersections.1 .1);
        ctx.line_to(points.1 .0, points.1 .1);
        ctx.stroke();

        ctx.set_stroke_style(&JsValue::from_str("#ff0000"));
        ctx.begin_path();
        let _ = ctx.ellipse(
            intersections.1 .0,
            intersections.1 .1,
            2.0,
            2.0,
            0.0,
            0.0,
            2.0 * PI,
        );
        ctx.stroke();
    }

    fn compute_points(&mut self) {
        for _ in 0..ITERATIONS_PER_FRAME {
            let intersections = get_intersection(&self.rotors.0, &self.rotors.1);
            self.points
                .push_back((intersections.1 .0, intersections.1 .1));
            self.rotors.0.advance();
            self.rotors.1.advance();
        }
    }

    fn render_line(&mut self, ctx: &CanvasRenderingContext2d) {
        ctx.set_stroke_style(&JsValue::from(self.color.clone()));
        let (x, y) = self.points.pop_front().unwrap();
        ctx.begin_path();
        ctx.move_to(x, y);
        let last = self.points.pop_back().unwrap();
        while let Some((x, y)) = self.points.pop_front() {
            ctx.line_to(x, y);
        }
        let (x, y) = last;
        ctx.line_to(x, y);
        ctx.stroke();
        self.points.push_back(last);
        self.compute_points();
    }
}
