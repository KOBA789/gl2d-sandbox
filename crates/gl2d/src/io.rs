use nalgebra::Vector2;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Io {
    pub(crate) screen_size: Vector2<u32>,
    pub(crate) pixel_ratio: f32,
    pub(crate) mouse: Vector2<f32>,
    pub(crate) wheel: Vector2<f32>,
    pub(crate) wheel_pinch: f32,
}

#[wasm_bindgen]
impl Io {
    #[allow(clippy::new_without_default)]
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            screen_size: Vector2::new(1, 1),
            pixel_ratio: 1.0,
            mouse: Vector2::zeros(),
            wheel: Vector2::zeros(),
            wheel_pinch: 0.0,
        }
    }

    #[wasm_bindgen(getter = wheelX)]
    pub fn wheel_x(&self) -> f32 {
        self.wheel.x
    }

    #[wasm_bindgen(setter = wheelX)]
    pub fn set_wheel_x(&mut self, wheel_x: f32) {
        self.wheel.x = wheel_x;
    }

    #[wasm_bindgen(getter = wheelY)]
    pub fn wheel_y(&self) -> f32 {
        self.wheel.y
    }

    #[wasm_bindgen(setter = wheelY)]
    pub fn set_wheel_y(&mut self, wheel_y: f32) {
        self.wheel.y = wheel_y;
    }

    #[wasm_bindgen(getter)]
    pub fn pinch(&self) -> f32 {
        self.wheel.x
    }

    #[wasm_bindgen(setter)]
    pub fn set_pinch(&mut self, pinch: f32) {
        self.wheel_pinch = pinch;
    }

    #[wasm_bindgen(getter = mouseX)]
    pub fn mouse_x(&self) -> f32 {
        self.mouse.x
    }

    #[wasm_bindgen(setter = mouseX)]
    pub fn set_mouse_x(&mut self, mouse_x: f32) {
        self.mouse.x = mouse_x;
    }

    #[wasm_bindgen(getter = mouseY)]
    pub fn mouse_y(&self) -> f32 {
        self.mouse.y
    }

    #[wasm_bindgen(setter = mouseY)]
    pub fn set_mouse_y(&mut self, mouse_y: f32) {
        self.mouse.y = mouse_y;
    }

    #[wasm_bindgen(js_name = setScreenSize)]
    pub fn set_screen_size(&mut self, x: u32, y: u32, pixel_ratio: f32) {
        self.screen_size = Vector2::new(x, y);
        self.pixel_ratio = pixel_ratio;
    }

    pub fn reset(&mut self) {
        self.wheel = Vector2::zeros();
        self.wheel_pinch = 0.0;
    }
}
