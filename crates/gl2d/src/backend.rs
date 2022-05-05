use crate::draw_list::DrawContext;

use super::draw_list::DrawList;
use anyhow::Result;
use glow::{Buffer, Context, HasContext, UniformLocation, WebFramebufferKey, WebProgramKey};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct GlowBackend {
    gl: Context,
    vbo: Buffer,
    ebo: Buffer,
    fbo: WebFramebufferKey,
    default_material: Material,
    text_material: Material,
}

fn glow_error(s: String) -> anyhow::Error {
    anyhow::anyhow!("Glow Error: {}", s)
}

#[wasm_bindgen]
impl GlowBackend {
    #[wasm_bindgen(constructor)]
    pub fn from_webgl(webgl_context: web_sys::WebGl2RenderingContext) -> Self {
        let gl = glow::Context::from_webgl2_context(webgl_context);
        Self::new(gl).unwrap()
    }
}

impl GlowBackend {
    pub fn new(gl: Context) -> Result<Self> {
        unsafe {
            let vao = gl.create_vertex_array().map_err(glow_error)?;
            gl.bind_vertex_array(Some(vao));
            gl.enable(glow::BLEND);
            gl.blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
            let default_material = Material::new(
                &gl,
                include_str!("shaders/default.vert"),
                include_str!("shaders/default.frag"),
            )?;
            let text_material = Material::new(
                &gl,
                include_str!("shaders/text.vert"),
                include_str!("shaders/text.frag"),
            )?;
            let vbo = gl.create_buffer().map_err(glow_error)?;
            let ebo = gl.create_buffer().map_err(glow_error)?;
            let fbo = gl.create_framebuffer().map_err(glow_error)?;
            gl.bind_framebuffer(glow::FRAMEBUFFER, fbo.into());
            let color_buffer = gl.create_texture().map_err(glow_error)?;
            gl.bind_texture(glow::TEXTURE_2D, color_buffer.into());
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                color_buffer.into(),
                0,
            );
            gl.bind_framebuffer(glow::FRAMEBUFFER, None);
            Ok(Self {
                gl,
                vbo,
                ebo,
                fbo,
                default_material,
                text_material,
            })
        }
    }

    pub fn draw(&mut self, draw_context: &DrawContext, draw_list: &DrawList) -> Result<()> {
        let w = draw_context.screen_size.x as f32;
        let h = draw_context.screen_size.y as f32;
        let scale = draw_context.scale;
        let translate = draw_context.translate;
        let sx = (2. / w) * scale;
        let sy = (2. / h) * scale;
        let npx = 2. * translate.x / w + 1. / w;
        let npy = -2. * translate.y / h + 1. / h;
        #[rustfmt::skip]
        let projection = [
            sx, 0., 0., 0.,
            0., -sy, 0., 0.,
            0., 0., -1., 0.,
            npx - 1., npy + 1., 0., 1.,
        ];
        let vertices = draw_list.vertices();
        let indices = draw_list.indices();
        unsafe {
            self.gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vbo));
            self.gl.buffer_data_u8_slice(
                glow::ARRAY_BUFFER,
                bytemuck::cast_slice(vertices),
                glow::STREAM_DRAW,
            );
            self.gl
                .bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(self.ebo));
            self.gl.buffer_data_u8_slice(
                glow::ELEMENT_ARRAY_BUFFER,
                bytemuck::cast_slice(indices),
                glow::STREAM_DRAW,
            );
            self.default_material.prepare(&self.gl, &projection);
            let width = ((draw_context.screen_size.x as f32) * draw_context.pixel_ratio) as i32;
            let height = ((draw_context.screen_size.y as f32) * draw_context.pixel_ratio) as i32;
            self.gl.viewport(0, 0, width, height);
            self.gl.clear_color(
                draw_context.bg_color.x,
                draw_context.bg_color.y,
                draw_context.bg_color.z,
                draw_context.bg_color.w,
            );
            self.gl
                .clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            for cmd in &draw_list.cmds {
                if cmd.is_text {
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, self.fbo.into());
                    self.gl.tex_image_2d(
                        glow::TEXTURE_2D,
                        0,
                        glow::RGBA as i32,
                        width,
                        height,
                        0,
                        glow::RGBA,
                        glow::UNSIGNED_BYTE,
                        None,
                    );
                    self.gl.blend_func(glow::SRC_ALPHA, glow::DST_ALPHA);
                    let start = cmd.idx_offset + 6;
                    let count = (cmd.num_of_elems - 2) * 3;
                    self.gl.draw_elements(
                        glow::TRIANGLES,
                        count as i32,
                        glow::UNSIGNED_INT,
                        (start * std::mem::size_of::<u32>()) as i32,
                    );
                    #[rustfmt::skip]
                    self.text_material.prepare(&self.gl, &[
                        1., 0., 0., 0.,
                        0., 1., 0., 0.,
                        0., 0., 1., 0.,
                        0., 0., 0., 1.,
                    ]);
                    self.gl.bind_framebuffer(glow::FRAMEBUFFER, None);
                    // draw texture
                    self.gl
                        .blend_func(glow::SRC_ALPHA, glow::ONE_MINUS_SRC_ALPHA);
                    let start = cmd.idx_offset;
                    let count = 6;
                    self.gl.draw_elements(
                        glow::TRIANGLES,
                        count as i32,
                        glow::UNSIGNED_INT,
                        (start * std::mem::size_of::<u32>()) as i32,
                    );

                    // reset
                    self.default_material.prepare(&self.gl, &projection);
                } else {
                    let start = cmd.idx_offset;
                    let count = cmd.num_of_elems * 3;
                    self.gl.draw_elements(
                        glow::TRIANGLES,
                        count as i32,
                        glow::UNSIGNED_INT,
                        (start * std::mem::size_of::<u32>()) as i32,
                    );
                }
            }
            self.gl.flush();
        }
        Ok(())
    }
}

struct Material {
    program: WebProgramKey,
    projection_location: UniformLocation,
    position_location: u32,
    color_location: u32,
}

impl Material {
    fn new(gl: &Context, vertex_shader_source: &str, fragment_shader_source: &str) -> Result<Self> {
        unsafe {
            let program = gl.create_program().map_err(glow_error)?;
            let vertex_shader = gl.create_shader(glow::VERTEX_SHADER).map_err(glow_error)?;
            gl.shader_source(vertex_shader, vertex_shader_source);
            gl.compile_shader(vertex_shader);
            if !gl.get_shader_compile_status(vertex_shader) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_shader_info_log(vertex_shader)
                ));
            }
            gl.attach_shader(program, vertex_shader);
            let fragment_shader = gl
                .create_shader(glow::FRAGMENT_SHADER)
                .map_err(glow_error)?;
            gl.shader_source(fragment_shader, fragment_shader_source);
            gl.compile_shader(fragment_shader);
            if !gl.get_shader_compile_status(fragment_shader) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_shader_info_log(fragment_shader)
                ));
            }
            gl.attach_shader(program, fragment_shader);
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(anyhow::anyhow!(
                    "Glow Error: {}",
                    gl.get_program_info_log(program)
                ));
            }
            gl.use_program(Some(program));
            gl.detach_shader(program, vertex_shader);
            gl.delete_shader(vertex_shader);
            gl.detach_shader(program, fragment_shader);
            gl.delete_shader(fragment_shader);

            let projection_location = gl
                .get_uniform_location(program, "projection")
                .ok_or_else(|| anyhow::anyhow!("No projection uniform variable"))?;
            let position_location = gl
                .get_attrib_location(program, "vert_position")
                .ok_or_else(|| anyhow::anyhow!("No vert_position attribute"))?;
            let color_location = gl
                .get_attrib_location(program, "vert_color")
                .ok_or_else(|| anyhow::anyhow!("No vert_color attribute"))?;

            Ok(Self {
                program,
                projection_location,
                position_location,
                color_location,
            })
        }
    }

    fn prepare(&self, gl: &Context, projection: &[f32; 16]) {
        unsafe {
            gl.use_program(self.program.into());
            let stride = (8 * std::mem::size_of::<f32>()) as i32;
            gl.enable_vertex_attrib_array(self.position_location);
            gl.vertex_attrib_pointer_f32(self.position_location, 4, glow::FLOAT, false, stride, 0);
            gl.enable_vertex_attrib_array(self.color_location);
            gl.vertex_attrib_pointer_f32(
                self.color_location,
                4,
                glow::FLOAT,
                false,
                stride,
                (4 * std::mem::size_of::<f32>()) as i32,
            );
            gl.uniform_matrix_4_f32_slice(Some(&self.projection_location), false, projection);
        }
    }
}
