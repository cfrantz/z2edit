use gl;
use imgui;
use std;

pub fn new_blank_image(width: u32, height: u32) -> imgui::TextureId {
    let pixels = vec![0u32; (width * height) as usize];
    new_image(width, height, &pixels)
}

pub fn new_image(width: u32, height: u32, pixels: &[u32]) -> imgui::TextureId {
    let mut image = 0u32;
    let pixels = pixels.as_ptr() as *const std::ffi::c_void;
    unsafe {
        gl::Enable(gl::TEXTURE_2D);
        gl::GenTextures(1, &mut image as *mut u32);
        gl::BindTexture(gl::TEXTURE_2D, image);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            gl::RGB as i32,
            width as i32,
            height as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_INT_8_8_8_8_REV,
            pixels,
        );
    }
    imgui::TextureId::from(image as usize)
}

pub fn update_image(image: imgui::TextureId, x: u32, y: u32, w: u32, h: u32, pixels: &[u32]) {
    let pixels = pixels.as_ptr() as *const std::ffi::c_void;
    unsafe {
        gl::Enable(gl::TEXTURE_2D);
        gl::BindTexture(gl::TEXTURE_2D, image.id() as u32);
        gl::TexSubImage2D(
            gl::TEXTURE_2D,
            0,
            x as i32,
            y as i32,
            w as i32,
            h as i32,
            gl::RGBA,
            gl::UNSIGNED_INT_8_8_8_8_REV,
            pixels,
        );
    }
}

pub fn delete_image(image: imgui::TextureId) {
    unsafe {
        let id = image.id() as u32;
        gl::DeleteTextures(1, &id as *const u32);
    }
}

#[derive(Debug)]
pub struct Image {
    pub id: imgui::TextureId,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u32>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        let pixels = vec![0u32; (width * height) as usize];
        let id = new_image(width, height, &pixels);
        Image {
            id: id,
            width: width,
            height: height,
            pixels: pixels,
        }
    }
    pub fn update(&self) {
        update_image(self.id, 0, 0, self.width, self.height, &self.pixels);
    }
    pub fn draw(&self, scale: f32, ui: &imgui::Ui) {
        let w = self.width as f32 * scale;
        let h = self.height as f32 * scale;
        imgui::Image::new(self.id, [w, h]).build(ui);
    }
    pub fn draw_at(&self, position: [f32; 2], scale: f32, ui: &imgui::Ui) {
        ui.set_cursor_pos(position);
        self.draw(scale, ui);
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        delete_image(self.id);
    }
}

pub fn clear_screen(color: &[f32; 3]) {
    let [r, g, b] = color;
    unsafe {
        gl::ClearColor(*r, *g, *b, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

const R_SHIFT: usize = 0;
const G_SHIFT: usize = 8;
const B_SHIFT: usize = 16;
const A_SHIFT: usize = 24;

pub fn color_as_f32(color: u32) -> [f32; 4] {
    [
        ((color >> R_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> G_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> B_SHIFT) & 0xFF) as f32 / 255.0,
        ((color >> A_SHIFT) & 0xFF) as f32 / 255.0,
    ]
}

pub fn color_as_u8(color: u32) -> [u8; 4] {
    [
        ((color >> R_SHIFT) & 0xFF) as u8,
        ((color >> G_SHIFT) & 0xFF) as u8,
        ((color >> B_SHIFT) & 0xFF) as u8,
        ((color >> A_SHIFT) & 0xFF) as u8,
    ]
}

pub fn color_to_f32(color: u32, fcol: &mut [f32; 4]) {
    fcol[0] = ((color >> R_SHIFT) & 0xFF) as f32 / 255.0;
    fcol[1] = ((color >> G_SHIFT) & 0xFF) as f32 / 255.0;
    fcol[2] = ((color >> B_SHIFT) & 0xFF) as f32 / 255.0;
    fcol[3] = ((color >> A_SHIFT) & 0xFF) as f32 / 255.0;
}

pub fn f32_to_color(fcol: &[f32; 4]) -> u32 {
    let r = (fcol[0] * 255.0) as u32;
    let g = (fcol[1] * 255.0) as u32;
    let b = (fcol[2] * 255.0) as u32;
    let a = (fcol[3] * 255.0) as u32;
    (r << R_SHIFT) | (g << G_SHIFT) | (b << B_SHIFT) | (a << A_SHIFT)
}
