use anyhow::{anyhow, Result};

use imgui::{ConfigFlags, Context};
use imgui_glow_renderer::glow;
use imgui_glow_renderer::glow::HasContext;
use imgui_glow_renderer::AutoRenderer;
use imgui_sdl2_support::SdlPlatform;
use pyo3::prelude::*;
use sdl2::{
    event::Event,
    video::{GLProfile, SwapInterval, Window},
    EventPump,
};
use send_wrapper::SendWrapper;

use crate::{Image, JsonStyle};

// Create a new glow context.
fn glow_context(window: &Window) -> glow::Context {
    unsafe {
        glow::Context::from_loader_function(|s| window.subsystem().gl_get_proc_address(s) as _)
    }
}

#[pyclass(unsendable)]
pub struct Framework {
    window: SendWrapper<Window>,
    _gl_context: sdl2::video::GLContext,
    platform: SdlPlatform,
    event_pump: EventPump,
    renderer: SendWrapper<AutoRenderer>,
    imgui: SendWrapper<Context>,
    dpi: f32,
    #[pyo3(get, set)]
    background: [f32; 3],
}

struct ClipboardBackend(sdl2::clipboard::ClipboardUtil);

impl imgui::ClipboardBackend for ClipboardBackend {
    fn get(&mut self) -> Option<String> {
        if self.0.has_clipboard_text() {
            self.0.clipboard_text().ok()
        } else {
            None
        }
    }
    fn set(&mut self, value: &str) {
        let _ = self.0.set_clipboard_text(value);
    }
}

#[pyclass(unsendable)]
#[repr(transparent)]
pub struct UiContext {
    pub ui: &'static imgui::Ui,
}

impl UiContext {
    pub fn new(ui: &imgui::Ui) -> Self {
        unsafe {
            // SAFETY: UiContext may not be held across frames.
            Self {
                ui: std::mem::transmute::<&imgui::Ui, &'static imgui::Ui>(ui),
            }
        }
    }
}

impl Framework {
    // FIXME: re-evaluate this when upgrading imgui.
    fn handle_modifier_bug(ui: &imgui::Ui) {
        use imgui::Key;
        let io = unsafe {
            // SAFETY: No one else is touching `io` at this time.
            #[allow(mutable_transmutes)]
            std::mem::transmute::<&imgui::Io, &mut imgui::Io>(ui.io())
        };
        if ui.is_key_down(Key::LeftShift) || ui.is_key_down(Key::RightShift) {
            io.key_shift = true;
        }
        if ui.is_key_down(Key::LeftAlt) || ui.is_key_down(Key::RightAlt) {
            io.key_alt = true;
        }
        if ui.is_key_down(Key::LeftCtrl) || ui.is_key_down(Key::RightCtrl) {
            io.key_ctrl = true;
        }
        if ui.is_key_down(Key::LeftSuper) || ui.is_key_down(Key::RightSuper) {
            io.key_super = true;
        }
    }
}

#[pymethods]
impl Framework {
    #[new]
    pub fn new(name: &str, width: u32, height: u32) -> Result<Self> {
        let sdl = sdl2::init().map_err(|e| anyhow!("SDL init: {e}"))?;
        let video_subsystem = sdl.video().map_err(|e| anyhow!("SDL video: {e}"))?;

        let (dpi, _hdpi, _vdpi) = video_subsystem
            .display_dpi(0)
            .map_err(|e| anyhow!("SDL display_dpi: {e}"))?;

        /* hint SDL to initialize an OpenGL 3.3 core profile context */
        let gl_attr = video_subsystem.gl_attr();

        gl_attr.set_context_version(3, 3);
        gl_attr.set_context_profile(GLProfile::Core);

        /* create a new window, be sure to call opengl method on the builder when using glow! */
        let window = video_subsystem
            .window(name, width, height)
            .allow_highdpi()
            .opengl()
            .position_centered()
            .resizable()
            .build()
            .unwrap();

        /* create a new OpenGL context and make it current */
        let _gl_context = window.gl_create_context().unwrap();
        window.gl_make_current(&_gl_context).unwrap();

        /* enable vsync to cap framerate */
        video_subsystem
            .gl_set_swap_interval(SwapInterval::VSync)
            .map_err(|e| anyhow!("SDL swap_interval: {e}"))?;

        /* create new glow and imgui contexts */
        let gl = glow_context(&window);

        /* create context */
        let mut imgui = Context::create();

        /* disable creation of files on disc */
        imgui.set_ini_filename(None);
        imgui.set_log_filename(None);

        /* setup platform and renderer, and fonts to imgui */
        imgui
            .fonts()
            .add_font(&[imgui::FontSource::DefaultFontData { config: None }]);

        imgui.io_mut().config_flags |= ConfigFlags::DOCKING_ENABLE;
        imgui.set_clipboard_backend(ClipboardBackend(window.subsystem().clipboard()));

        /* create platform and renderer */
        let platform = SdlPlatform::new(&mut imgui);
        let renderer = AutoRenderer::new(gl, &mut imgui).unwrap();
        Image::initialize(renderer.gl_context());

        /* start main loop */
        let event_pump = sdl.event_pump().unwrap();

        Ok(Framework {
            window: SendWrapper::new(window),
            _gl_context,
            platform,
            event_pump,
            renderer: SendWrapper::new(renderer),
            imgui: SendWrapper::new(imgui),
            dpi,
            background: [0.0625, 0.0625, 0.0625],
        })
    }

    pub fn set_scale(&mut self, scale: f32) {
        let mut scale = scale;
        if scale == 0.0 {
            scale = self.dpi / 96.0;
        }
        let style = self.imgui.style_mut();
        style.scale_all_sizes(scale);
        self.imgui.io_mut().font_global_scale = scale;
    }

    #[getter]
    pub fn get_style(&self) -> JsonStyle {
        JsonStyle::from(self.imgui.style())
    }

    #[setter]
    pub fn set_style(&mut self, style: &JsonStyle) {
        *self.imgui.style_mut() = imgui::Style::from(style);
    }

    pub fn prepare_frame(&mut self) -> Option<UiContext> {
        for event in self.event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            self.platform.handle_event(&mut self.imgui, &event);

            if let Event::Quit { .. } = event {
                return None;
            }
            /*
            if let Event::KeyDown { ref keymod, .. } = event {
                Self::handle_key_modifier(self.imgui.io_mut(), keymod, true);
            }
            if let Event::KeyUp { ref keymod, .. } = event {
                Self::handle_key_modifier(self.imgui.io_mut(), keymod, false);
            }
            */
        }

        /* call prepare_frame before calling imgui.new_frame() */
        self.platform
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = self.imgui.new_frame();
        Self::handle_modifier_bug(ui);
        // Wrap the `ui` for python.
        Some(UiContext::new(ui))
    }

    pub fn render_frame(&mut self, py: Python<'_>) {
        py.allow_threads(|| {
            let draw_data = self.imgui.render();
            unsafe {
                let gl = self.renderer.gl_context();
                gl.clear_color(
                    self.background[0],
                    self.background[1],
                    self.background[2],
                    1.0,
                );
                gl.clear(glow::COLOR_BUFFER_BIT);
            };
            if draw_data.draw_lists_count() > 0 {
                self.renderer.render(draw_data).unwrap();
            }
            self.window.gl_swap_window();
        });
    }
}
