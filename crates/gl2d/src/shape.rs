use nalgebra::{Vector2, Vector4};

use crate::draw_list::{Color, DrawList, Vert, DrawContext};

pub struct LineParams {
    half_thickness: f32,
    cap_segments: Vec<Vector2<f32>>,
}

impl LineParams {
    pub fn new(scale: f32, thickness: f32) -> Self {
        let resolution = thickness * scale;
        let cap_segment_count = if resolution <= 1.0 {
            0
        } else {
            (resolution * 1.5).ceil() as usize
        };
        let half_thickness = thickness * 0.5;
        let cap_segments = (0..=cap_segment_count)
            .map(|i| {
                let rad = i as f32 * 2.0 / cap_segment_count as f32 * std::f32::consts::PI;
                Vector2::new(rad.cos(), rad.sin()).scale(half_thickness)
            })
            .collect();
        Self {
            half_thickness,
            cap_segments,
        }
    }

    fn vtx_count(&self) -> usize {
        4 + self.cap_segments.len()
    }

    fn idx_count(&self) -> usize {
        (2 + self.cap_segments.len()) * 3
    }
}

impl DrawList {
    pub fn add_line_with_params(
        &mut self,
        p1: Vector2<f32>,
        p2: Vector2<f32>,
        col: Color,
        params: &LineParams,
    ) {
        self.reserve(params.idx_count(), params.vtx_count());

        let mut d = p2 - p1;
        d.try_normalize_mut(0.);
        d.scale_mut(params.half_thickness);

        let v0 = self.push_vert(Vert {
            pos: Vector4::new(p1.x + d.y, p1.y - d.x, 0., 1.),
            col,
        });
        let v1 = self.push_vert(Vert {
            pos: Vector4::new(p2.x + d.y, p2.y - d.x, 0., 1.),
            col,
        });
        let v2 = self.push_vert(Vert {
            pos: Vector4::new(p2.x - d.y, p2.y + d.x, 0., 1.),
            col,
        });
        let v3 = self.push_vert(Vert {
            pos: Vector4::new(p1.x - d.y, p1.y + d.x, 0., 1.),
            col,
        });
        self.push_elem(v0, v1, v2);
        self.push_elem(v0, v2, v3);

        if !params.cap_segments.is_empty() {
            let mut v_t = v1;
            let mut v_b = v3;
            let horizon = Vector2::new(-d.y, d.x);
            for r in params.cap_segments.iter() {
                if r.perp(&horizon) < 0. {
                    let xy = p1 + r;
                    let v = self.push_vert(Vert {
                        pos: Vector4::new(xy.x, xy.y, 0., 1.),
                        col,
                    });
                    self.push_elem(v0, v_b, v);
                    v_b = v;
                    v_t = v1;
                } else {
                    let xy = p2 + r;
                    let v = self.push_vert(Vert {
                        pos: Vector4::new(xy.x, xy.y, 0., 1.),
                        col,
                    });
                    self.push_elem(v_t, v2, v);
                    v_t = v;
                    v_b = v3;
                }
            }
        }
    }

    pub fn add_line(&mut self, ctx: &DrawContext, p1: Vector2<f32>, p2: Vector2<f32>, col: Color, thickness: f32) {
        let resolution = thickness * ctx.scale;
        let cap_segment_count = if resolution <= 1.0 {
            0
        } else {
            (resolution * 1.5).ceil() as usize
        };
        let vtx_count = 4 + cap_segment_count;
        let idx_count = (2 + cap_segment_count) * 3;
        self.reserve(idx_count, vtx_count);

        let half_thickness = thickness * 0.5;
        let mut d = p2 - p1;
        d.try_normalize_mut(0.);
        d.scale_mut(half_thickness);

        let v0 = self.push_vert(Vert {
            pos: Vector4::new(p1.x + d.y, p1.y - d.x, 0., 1.),
            col,
        });
        let v1 = self.push_vert(Vert {
            pos: Vector4::new(p2.x + d.y, p2.y - d.x, 0., 1.),
            col,
        });
        let v2 = self.push_vert(Vert {
            pos: Vector4::new(p2.x - d.y, p2.y + d.x, 0., 1.),
            col,
        });
        let v3 = self.push_vert(Vert {
            pos: Vector4::new(p1.x - d.y, p1.y + d.x, 0., 1.),
            col,
        });
        self.push_elem(v0, v1, v2);
        self.push_elem(v0, v2, v3);

        if cap_segment_count > 0 {
            let mut v_t = v1;
            let mut v_b = v3;
            let horizon = Vector2::new(-d.y, d.x);
            for i in 0..=cap_segment_count {
                let rad = i as f32 * 2.0 / cap_segment_count as f32 * std::f32::consts::PI;
                let r = Vector2::new(rad.cos(), rad.sin()).scale(half_thickness);
                if r.perp(&horizon) < 0. {
                    let xy = p1 + r;
                    let v = self.push_vert(Vert {
                        pos: Vector4::new(xy.x, xy.y, 0., 1.),
                        col,
                    });
                    self.push_elem(v0, v_b, v);
                    v_b = v;
                    v_t = v1;
                } else {
                    let xy = p2 + r;
                    let v = self.push_vert(Vert {
                        pos: Vector4::new(xy.x, xy.y, 0., 1.),
                        col,
                    });
                    self.push_elem(v_t, v2, v);
                    v_t = v;
                    v_b = v3;
                }
            }
        }
    }

    pub fn add_circle(&mut self, ctx: &DrawContext, p: Vector2<f32>, r: f32, col: Color, thickness: f32) {
        let resolution = thickness * ctx.scale;
        let half_thickness = thickness * 0.5;
        let segment_count = (r + resolution).ceil() as usize;
        let vtx_count = 2 * segment_count;
        let idx_count = (2 * segment_count) * 3;
        self.reserve(idx_count, vtx_count);

        let r_o = r + half_thickness;
        let r_i = r - half_thickness;
        let mut v_o0 = self.push_vert(Vert {
            pos: Vector4::new(p.x + r_o, p.y, 0., 1.),
            col,
        });
        let mut v_i0 = self.push_vert(Vert {
            pos: Vector4::new(p.x + r_i, p.y, 0., 1.),
            col,
        });
        for i in 1..=segment_count {
            let rad = i as f32 * 2.0 / segment_count as f32 * std::f32::consts::PI;
            let v = Vector2::new(rad.cos(), rad.sin());
            let xy_o = p + v.scale(r_o);
            let v_o1 = self.push_vert(Vert {
                pos: Vector4::new(xy_o.x, xy_o.y, 0., 1.),
                col,
            });
            let xy_i = p + v.scale(r_i);
            let v_i1 = self.push_vert(Vert {
                pos: Vector4::new(xy_i.x, xy_i.y, 0., 1.),
                col,
            });
            self.push_elem(v_o0, v_i0, v_o1);
            self.push_elem(v_o1, v_i1, v_i0);
            v_o0 = v_o1;
            v_i0 = v_i1;
        }
    }

    #[allow(clippy::many_single_char_names)]
    pub fn add_square(&mut self, p: Vector2<f32>, size: f32, col: Color) {
        let half_size = size * 0.5;
        self.reserve(6, 4);
        let a = self.push_vert(Vert {
            pos: Vector4::new(p.x - half_size, p.y - half_size, 0., 1.),
            col,
        });
        let b = self.push_vert(Vert {
            pos: Vector4::new(p.x + half_size, p.y - half_size, 0., 1.),
            col,
        });
        let c = self.push_vert(Vert {
            pos: Vector4::new(p.x - half_size, p.y + half_size, 0., 1.),
            col,
        });
        let d = self.push_vert(Vert {
            pos: Vector4::new(p.x + half_size, p.y + half_size, 0., 1.),
            col,
        });
        self.push_elem(a, b, c);
        self.push_elem(b, c, d);
    }
}
