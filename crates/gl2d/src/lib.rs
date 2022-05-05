use draw_list::DrawContext;
use nalgebra::Vector2;
use owned_ttf_parser::Face;
use wasm_bindgen::prelude::*;

use crate::draw_list::{Color, DrawList};

pub use crate::backend::GlowBackend;
use crate::glyph::Glyph;
pub use crate::io::Io;

mod backend;
mod draw_list;
mod glyph;
mod io;
mod shape;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

const FONT_DATA: &[u8] = include_bytes!("../../../fonts/HackGen-Regular.ttf");
const TEXT: &str = include_str!("../../../data/hashire_merosu.txt");

#[wasm_bindgen]
pub struct Gl2d {
    backend: backend::GlowBackend,
    transform: Transform,
    draw_list: DrawList,
    draw_context: DrawContext,

    glyphs: Vec<Vec<(Glyph, f32)>>,
}

#[wasm_bindgen]
impl Gl2d {
    #[wasm_bindgen(constructor)]
    pub fn new(backend: GlowBackend) -> Self {
        let draw_list = DrawList::new();
        let mut draw_context = DrawContext::new(Vector2::new(1000, 1000));
        draw_context.bg_color = Color::new(0., 0., 0., 0.);
        let transform = Default::default();

        let face = Face::from_slice(FONT_DATA, 0).unwrap();
        let glyphs = TEXT
            .split('\n')
            .map(|line| {
                line.chars()
                    .filter_map(|ch| {
                        if let Some(glyph_id) = face.glyph_index(ch) {
                            if let Some(hor_advance) = face.glyph_hor_advance(glyph_id) {
                                let glyph = Glyph::new(&face, glyph_id);
                                Some((glyph, hor_advance as f32))
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .collect();
        Self {
            backend,
            transform,
            draw_list,
            draw_context,

            glyphs,
        }
    }

    pub fn begin_frame(&mut self, io: &mut io::Io) {
        self.transform.screen_size = io.screen_size;
        let pixel_ratio = io.pixel_ratio;
        let pan = -io.wheel;
        let origin = io.mouse;
        let mut zoom = 1. - io.wheel_pinch * 0.02;
        if self.transform.scale * zoom < 0.1 {
            zoom = 0.1 / self.transform.scale;
        } else if self.transform.scale * zoom > 16.0 {
            zoom = 16.0 / self.transform.scale;
        }
        self.transform.pan_zoom(pan, origin, zoom);
        io.reset();
        self.draw_list.clear();
        self.draw_context.pixel_ratio = pixel_ratio;
        self.draw_context.scale = self.transform.scale;
        self.draw_context.translate = self.transform.translate;
        self.draw_context.screen_size = self.transform.screen_size;
    }

    pub fn draw(&mut self) {
        self.draw_list.new_text_layer(Color::new(0., 0., 0., 1.0));

        let scale = 0.2f32;
        let line_height = 1000.0 * scale;
        for (lineno, line) in self.glyphs.iter().enumerate() {
            let mut x = 0f32;
            for (glyph, hor_advance) in line {
                self.draw_list.add_glyph(
                    &self.draw_context,
                    Vector2::new(x, line_height * (lineno + 1) as f32),
                    scale,
                    glyph,
                );
                x += hor_advance * scale;
            }
        }
        self.backend
            .draw(&self.draw_context, &self.draw_list)
            .unwrap();
    }
}

struct Transform {
    scale: f32,
    translate: Vector2<f32>,
    screen_size: Vector2<u32>,
}

impl Transform {
    fn pan_zoom(&mut self, pan: Vector2<f32>, origin: Vector2<f32>, zoom: f32) {
        self.translate = self.translate.scale(zoom) - origin.scale(zoom) + origin + pan;
        self.scale *= zoom;
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            scale: 1.,
            translate: Vector2::zeros(),
            screen_size: Vector2::new(1, 1),
        }
    }
}

#[wasm_bindgen]
pub fn license() -> String {
    include_str!("../../../fonts/LICENSE").to_string()
}
