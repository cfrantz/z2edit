use crate::errors::*;
use sdl2;
use std::cell::RefCell;
use std::path::{Path, PathBuf};

pub struct AppContext {
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub gl: sdl2::video::GLContext,
    pub event_pump: RefCell<sdl2::EventPump>,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
}

static mut APP_CONTEXT: Option<AppContext> = None;

impl AppContext {
    pub fn init(
        name: &str,
        width: u32,
        height: u32,
        config_dir: &Path,
        data_dir: &Path,
    ) -> Result<()> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;
        {
            let gl_attr = video.gl_attr();
            gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
            gl_attr.set_context_version(3, 0)
        }
        let window = video
            .window(name, width, height)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()?;

        let event_pump = sdl.event_pump()?;
        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        unsafe {
            APP_CONTEXT = Some(AppContext {
                sdl: sdl,
                video: video,
                window: window,
                gl: gl_context,
                event_pump: RefCell::new(event_pump),
                config_dir: config_dir.to_path_buf(),
                data_dir: data_dir.to_path_buf(),
            });
        }
        Ok(())
    }

    pub fn get() -> &'static AppContext {
        unsafe { APP_CONTEXT.as_ref().unwrap() }
    }
}
