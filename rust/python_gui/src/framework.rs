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

use crate::Image;

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

    pub fn prepare_frame(&mut self) -> Option<UiContext> {
        for event in self.event_pump.poll_iter() {
            /* pass all events to imgui platfrom */
            self.platform.handle_event(&mut self.imgui, &event);

            if let Event::Quit { .. } = event {
                return None;
            }
        }

        /* call prepare_frame before calling imgui.new_frame() */
        self.platform
            .prepare_frame(&mut self.imgui, &self.window, &self.event_pump);

        let ui = UiContext::new(self.imgui.new_frame());
        Some(ui)
    }

    pub fn render_frame(&mut self, py: Python<'_>) {
        py.allow_threads(|| {
            let draw_data = self.imgui.render();
            unsafe { self.renderer.gl_context().clear(glow::COLOR_BUFFER_BIT) };
            if draw_data.draw_lists_count() > 0 {
                self.renderer.render(draw_data).unwrap();
            }
            self.window.gl_swap_window();
        });
    }
}
