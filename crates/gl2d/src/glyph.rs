use nalgebra::{Vector2, Vector4};
use owned_ttf_parser::{Face, GlyphId, OutlineBuilder, Rect};

use crate::draw_list::{Color, DrawContext, DrawList, Vert};

#[derive(Debug, Default)]
pub struct Glyph {
    idx_buffer: Vec<u32>,
    vtx_buffer: Vec<Vert>,
    num_of_elems: usize,
    bounding_box: Option<Rect>,
}

impl Glyph {
    pub fn new(face: &Face, glyph_id: GlyphId) -> Glyph {
        let mut builder = GlyphBuilder::default();
        let rect = face.outline_glyph(glyph_id, &mut builder);
        let mut glyph = builder.build();
        glyph.bounding_box = rect;
        glyph
    }

    pub fn reserve(&mut self, idx_count: usize, vtx_count: usize) {
        self.idx_buffer.reserve(idx_count);
        self.vtx_buffer.reserve(vtx_count);
    }

    pub fn push_vert(&mut self, vert: Vert) -> u32 {
        let idx = self.vtx_buffer.len() as u32;
        self.vtx_buffer.push(vert);
        idx
    }

    pub fn push_elem(&mut self, a: u32, b: u32, c: u32) {
        self.idx_buffer.push(a);
        self.idx_buffer.push(b);
        self.idx_buffer.push(c);
        self.num_of_elems += 1;
    }
}

impl DrawList {
    pub fn add_glyph(
        &mut self,
        ctx: &DrawContext,
        position: Vector2<f32>,
        scale: f32,
        glyph: &Glyph,
    ) {
        if let Some(bb) = glyph.bounding_box {
            let glyph_left_top = Vector2::new(bb.x_min as f32, -bb.y_max as f32) * scale + position;
            let glyph_right_bottom =
                Vector2::new(bb.x_max as f32, bb.y_min as f32) * scale + position;
            let screen_left_top = (-ctx.translate).unscale(ctx.scale);
            let screen_right_bottom = (ctx.screen_size.cast() - ctx.translate).unscale(ctx.scale);
            let hit = glyph_left_top.x <= screen_right_bottom.x
                && glyph_right_bottom.x >= screen_left_top.x
                && glyph_left_top.y <= screen_right_bottom.y
                && glyph_right_bottom.y >= screen_left_top.y;
            if !hit {
                return;
            }
        }
        let vtx_buffer_len = self.vtx_buffer.len() as u32;
        self.idx_buffer
            .extend(glyph.idx_buffer.iter().map(|idx| idx + vtx_buffer_len));
        for &Vert { ref pos, col } in glyph.vtx_buffer.iter() {
            let xy = pos.xy() * scale + position;
            self.vtx_buffer.push(Vert {
                pos: Vector4::new(xy.x, xy.y, pos.z, pos.w),
                col,
            });
        }
        self.cmds.last_mut().unwrap().num_of_elems += glyph.num_of_elems;
    }
}

#[derive(Default)]
pub struct GlyphBuilder {
    first_point: Vector2<f32>,
    current_point: Vector2<f32>,
    contour_count: usize,
    glyph: Glyph,
}

impl GlyphBuilder {
    #[inline]
    pub fn add_triangle(&mut self, a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>, col: Color) {
        self.glyph.reserve(3, 3);
        let a = self.glyph.push_vert(Vert {
            pos: Vector4::new(a.x, a.y, 0., 1.),
            col,
        });
        let b = self.glyph.push_vert(Vert {
            pos: Vector4::new(b.x, b.y, 0., 1.),
            col,
        });
        let c = self.glyph.push_vert(Vert {
            pos: Vector4::new(c.x, c.y, 1., 1.),
            col,
        });
        self.glyph.push_elem(a, b, c);
    }

    #[inline]
    pub fn add_curve(&mut self, a: Vector2<f32>, b: Vector2<f32>, c: Vector2<f32>, col: Color) {
        self.glyph.reserve(3, 3);
        let a = self.glyph.push_vert(Vert {
            pos: Vector4::new(a.x, a.y, 0., 0.),
            col,
        });
        let b = self.glyph.push_vert(Vert {
            pos: Vector4::new(b.x, b.y, 0.5, 0.),
            col,
        });
        let c = self.glyph.push_vert(Vert {
            pos: Vector4::new(c.x, c.y, 1., 1.),
            col,
        });
        self.glyph.push_elem(a, b, c);
    }

    pub fn build(self) -> Glyph {
        self.glyph
    }
}

impl OutlineBuilder for GlyphBuilder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.first_point = Vector2::new(x, -y);
        self.current_point = self.first_point;
        self.contour_count = 0;
    }

    fn line_to(&mut self, x: f32, y: f32) {
        self.contour_count += 1;
        let new_point = Vector2::new(x, -y);
        if self.contour_count >= 2 {
            self.add_triangle(
                self.first_point,
                self.current_point,
                new_point,
                Color::new(0., 0., 1. / 255., 1.),
            );
        }
        self.current_point = new_point;
    }

    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.contour_count += 1;
        let new_point = Vector2::new(x, -y);
        if self.contour_count >= 2 {
            self.add_triangle(
                self.first_point,
                self.current_point,
                new_point,
                Color::new(0., 0., 1. / 255., 1.),
            );
        }
        self.add_curve(
            self.current_point,
            Vector2::new(x1, -y1),
            new_point,
            Color::new(0., 0., 1. / 255., 1.),
        );
        self.current_point = new_point;
    }

    fn curve_to(&mut self, _x1: f32, _y1: f32, _x2: f32, _y2: f32, _x: f32, _y: f32) {
        unimplemented!();
    }

    fn close(&mut self) {
        self.current_point = self.first_point;
        self.contour_count = 0;
    }
}
