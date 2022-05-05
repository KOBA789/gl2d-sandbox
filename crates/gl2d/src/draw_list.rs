use nalgebra::base::{Vector2, Vector4};

pub type Color = Vector4<f32>;

pub struct DrawContext {
    pub screen_size: Vector2<u32>,
    pub pixel_ratio: f32,
    pub translate: Vector2<f32>,
    pub scale: f32,
    pub bg_color: Color,
}

impl DrawContext {
    pub fn new(screen_size: Vector2<u32>) -> Self {
        Self {
            screen_size,
            pixel_ratio: 1.0,
            translate: Vector2::zeros(),
            scale: 1.,
            bg_color: Color::new(1., 1., 1., 1.),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawList {
    pub cmds: Vec<DrawCmd>,
    pub idx_buffer: Vec<u32>,
    pub vtx_buffer: Vec<Vert>,
}

impl DrawList {
    pub fn new() -> Self {
        Self {
            cmds: vec![DrawCmd::default()],
            idx_buffer: vec![],
            vtx_buffer: vec![],
        }
    }

    pub fn clear(&mut self) {
        self.cmds.clear();
        self.cmds.push(DrawCmd::default());
        self.idx_buffer.clear();
        self.vtx_buffer.clear();
    }

    pub fn new_layer(&mut self) {
        self.cmds.push(DrawCmd {
            idx_offset: self.idx_buffer.len(),
            vtx_offset: self.vtx_buffer.len(),
            num_of_elems: 0,
            is_text: false,
        });
    }

    pub fn new_text_layer(&mut self, col: Color) {
        self.cmds.push(DrawCmd {
            idx_offset: self.idx_buffer.len(),
            vtx_offset: self.vtx_buffer.len(),
            num_of_elems: 0,
            is_text: true,
        });
        let a = self.push_vert(Vert {
            pos: Vector4::new(-1., -1., 0., 1.),
            col,
        });
        let b = self.push_vert(Vert {
            pos: Vector4::new(1., -1., 0., 1.),
            col,
        });
        let c = self.push_vert(Vert {
            pos: Vector4::new(-1., 1., 0., 1.),
            col,
        });
        let d = self.push_vert(Vert {
            pos: Vector4::new(1., 1., 0., 1.),
            col,
        });
        self.push_elem(a, b, c);
        self.push_elem(b, c, d);
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
        self.cmds.last_mut().unwrap().num_of_elems += 1;
    }
}

impl Default for DrawList {
    fn default() -> Self {
        Self::new()
    }
}

impl DrawList {
    pub fn vertices(&self) -> &[f32] {
        unsafe {
            let len =
                std::mem::size_of::<Vert>() / std::mem::size_of::<f32>() * self.vtx_buffer.len();
            #[allow(clippy::transmute_ptr_to_ptr)]
            let ptr = std::mem::transmute::<*const Vert, *const f32>(self.vtx_buffer.as_ptr());
            &*std::ptr::slice_from_raw_parts(ptr, len)
        }
    }

    pub fn indices(&self) -> &[u32] {
        &self.idx_buffer
    }
}

#[derive(Debug, Clone)]
#[repr(C)]
pub struct Vert {
    pub pos: Vector4<f32>,
    pub col: Color,
}

#[derive(Debug, Clone, Default)]
pub struct DrawCmd {
    pub vtx_offset: usize,
    pub idx_offset: usize,
    pub num_of_elems: usize,
    pub is_text: bool,
}
