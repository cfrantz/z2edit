use anyhow::{anyhow, Result};
use imgui_glow_renderer::glow;
use imgui_glow_renderer::glow::HasContext;
use pyo3::prelude::*;
use sdl2::pixels::PixelFormatEnum;
use sdl2::surface::Surface;
use send_wrapper::SendWrapper;
use std::path::Path;
use std::rc::Rc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::OnceLock;
use zerocopy::{FromBytes, Immutable, IntoBytes};

use crate::UiContext;

#[derive(Clone, Copy, Debug, Default, IntoBytes, FromBytes, Immutable)]
#[repr(C)]
#[pyclass]
#[pyo3(get_all, set_all)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[pymethods]
impl Color {
    #[new]
    pub fn new(value: u32) -> Self {
        let [r, g, b, a] = value.to_le_bytes();
        Color { r, g, b, a }
    }

    #[staticmethod]
    pub fn from_f32(color: [f32; 4]) -> Self {
        let r = (color[0] * 255.0) as u8;
        let g = (color[1] * 255.0) as u8;
        let b = (color[2] * 255.0) as u8;
        let a = (color[3] * 255.0) as u8;
        Color { r, g, b, a }
    }

    pub fn to_u32(&self) -> u32 {
        u32::from_le_bytes([self.r, self.g, self.b, self.a])
    }

    pub fn to_f32(&self) -> [f32; 4] {
        [
            self.r as f32 / 255.0,
            self.g as f32 / 255.0,
            self.b as f32 / 255.0,
            self.a as f32 / 255.0,
        ]
    }

    pub fn average(&self) -> u32 {
        let r = self.r as u32;
        let b = self.g as u32;
        let g = self.b as u32;
        (r + g + b) / 3
    }

    pub fn blend(&self, other: Color) -> Color {
        let [r1, g1, b1, _] = self.to_f32();
        let [r2, g2, b2, a2] = other.to_f32();
        let a1 = 1.0 - a2;
        #[rustfmt::skip]
        let result = Color::from_f32([
            a1*r1 + a2*r2,
            a1*g1 + a2*g2,
            a1*b1 + a2*b2,
            1.0,
        ]);
        result
    }

    fn __repr__(&self) -> String {
        format!("Color(0x{:08x})", self.to_u32())
    }
}

static GL_CONTEXT: OnceLock<SendWrapper<Rc<glow::Context>>> = OnceLock::new();

#[pyclass(unsendable)]
pub struct Image {
    pub id: glow::Texture,
    #[pyo3(get, set)]
    pub width: u32,
    #[pyo3(get, set)]
    pub height: u32,
    #[pyo3(get, set)]
    pub pixels: Vec<Color>,
    needs_update: AtomicBool,
}

impl Image {
    pub(crate) fn initialize(context: &Rc<glow::Context>) {
        GL_CONTEXT
            .set(SendWrapper::new(Rc::clone(context)))
            .expect("Image already initialized");
    }

    fn gl() -> Rc<glow::Context> {
        Rc::clone(GL_CONTEXT.get().expect("Image not initialized"))
    }

    fn gl_new_image(width: u32, height: u32, pixels: &[u8]) -> glow::Texture {
        unsafe {
            let gl = Self::gl();
            gl.enable(glow::TEXTURE_2D);
            let texture = gl.create_texture().expect("create_texture");
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MIN_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_MAG_FILTER,
                glow::NEAREST as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_S,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_parameter_i32(
                glow::TEXTURE_2D,
                glow::TEXTURE_WRAP_T,
                glow::CLAMP_TO_EDGE as i32,
            );
            gl.tex_image_2d(
                glow::TEXTURE_2D,
                0,
                glow::SRGB8_ALPHA8 as i32,
                width as i32,
                height as i32,
                0,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                Some(pixels),
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
            texture
        }
    }

    fn gl_update_image(texture: glow::Texture, x: u32, y: u32, w: u32, h: u32, pixels: &[u8]) {
        unsafe {
            let gl = Self::gl();
            gl.enable(glow::TEXTURE_2D);
            gl.bind_texture(glow::TEXTURE_2D, Some(texture));
            gl.tex_sub_image_2d(
                glow::TEXTURE_2D,
                0,
                x as i32,
                y as i32,
                w as i32,
                h as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelUnpackData::Slice(pixels),
            );
            gl.bind_texture(glow::TEXTURE_2D, None);
        }
    }

    fn gl_delete_image(texture: glow::Texture) {
        unsafe {
            let gl = Self::gl();
            gl.delete_texture(texture);
        }
    }

    pub fn imgui_id(&self) -> imgui::TextureId {
        imgui::TextureId::new(self.id.0.get() as usize)
    }

    pub fn draw(&self, scale: f32, ui: &imgui::Ui) {
        if self.needs_update.load(Ordering::Relaxed) {
            self.update();
            self.needs_update.store(false, Ordering::Relaxed);
        }
        let w = self.width as f32 * scale;
        let h = self.height as f32 * scale;
        imgui::Image::new(self.imgui_id(), [w, h]).build(ui);
    }

    pub fn draw_at(&self, position: [f32; 2], scale: f32, ui: &imgui::Ui) {
        ui.set_cursor_pos(position);
        self.draw(scale, ui);
    }

    pub fn save_bmp<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let pixels = unsafe {
            // SAFETY: Surface::from_data requires a mutable reference, but we
            // aren't going to modify the data, so we use transmute to lie.
            #[allow(mutable_transmutes)]
            std::mem::transmute::<&[u8], &mut [u8]>(self.pixels.as_bytes())
        };
        let surface = Surface::from_data(
            pixels,
            self.width,
            self.height,
            self.width * 4,
            PixelFormatEnum::ABGR8888,
        )
        .map_err(|e| anyhow!("Surface::from_data: {e}"))?;
        surface
            .save_bmp(path)
            .map_err(|e| anyhow!("Surface::save_bmp: {e}"))?;
        Ok(())
    }

    pub fn load_bmp<P: AsRef<Path>>(path: P) -> Result<Image> {
        let surface = Surface::load_bmp(path).map_err(|e| anyhow!("Surface::load_bmp: {e}"))?;
        let surface = surface
            .convert_format(PixelFormatEnum::ABGR8888)
            .map_err(|e| anyhow!("Surface::convert_format: {e}"))?;
        surface.with_lock(|p| {
            let mut image = Image::new(surface.width(), surface.height());
            let w4 = image.width as usize * 4;
            let pitch = surface.pitch() as usize;
            let pixels = image.pixels.as_mut_bytes();
            for y in 0..(image.height as usize) {
                pixels[(y * w4)..((y + 1) * w4)]
                    .clone_from_slice(&p[(y * pitch)..((y + 1) * pitch)]);
            }
            image.needs_update.store(true, Ordering::Relaxed);
            Ok(image)
        })
    }
}

#[pymethods]
impl Image {
    #[new]
    pub fn new(width: u32, height: u32) -> Self {
        Self::with_color(width, height, Color::default())
    }

    #[staticmethod]
    pub fn with_color(width: u32, height: u32, color: Color) -> Self {
        let pixels = vec![color; (width * height) as usize];
        let id = Self::gl_new_image(width, height, pixels.as_bytes());
        Image {
            id,
            width,
            height,
            pixels,
            needs_update: AtomicBool::new(false),
        }
    }

    pub fn update(&self) {
        Self::gl_update_image(
            self.id,
            0,
            0,
            self.width,
            self.height,
            self.pixels.as_bytes(),
        );
    }

    pub fn overlay(&mut self, other: &Image, x0: u32, y0: u32) {
        for y in 0..other.height {
            if y + y0 > self.height {
                break;
            }
            for x in 0..other.width {
                if x + x0 > self.width {
                    break;
                }
                let i = ((y + y0) * self.width + x + x0) as usize;
                let j = (y * other.width + x) as usize;
                self.pixels[i] = self.pixels[i].blend(other.pixels[j]);
            }
        }
        self.needs_update.store(true, Ordering::Relaxed);
    }

    #[pyo3(name = "draw")]
    fn _draw(&self, scale: f32, ui: &UiContext) {
        self.draw(scale, ui.ui)
    }
    #[pyo3(name = "draw_at")]
    fn _draw_at(&self, position: [f32; 2], scale: f32, ui: &UiContext) {
        self.draw_at(position, scale, ui.ui)
    }
    #[pyo3(name = "save_bmp")]
    fn _save_bmp(&self, path: &str) -> Result<()> {
        self.save_bmp(path)
    }
    #[pyo3(name = "load_bmp")]
    #[staticmethod]
    fn _load_bmp(path: &str) -> Result<Image> {
        Self::load_bmp(path)
    }

    #[getter]
    fn get_id(&self) -> u32 {
        self.id.0.get()
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        let i = (y * self.width + x) as usize;
        self.pixels[i] = color;
        self.needs_update.store(true, Ordering::Relaxed);
    }

    pub fn get_pixel(&self, x: u32, y: u32) -> Color {
        let i = (y * self.width + x) as usize;
        self.pixels[i]
    }

}

impl Drop for Image {
    fn drop(&mut self) {
        Self::gl_delete_image(self.id);
    }
}
