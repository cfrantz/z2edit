use crate::errors::*;
use pyo3::prelude::*;
use sdl2;
use std::cell::RefCell;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

use crate::gui::app::App;
use crate::gui::preferences::Preferences;

#[derive(StructOpt, Debug)]
#[structopt(name = "z2edit")]
pub struct CommandlineArgs {
    #[structopt(short, long, default_value = "1280")]
    pub width: u32,
    #[structopt(short, long, default_value = "720")]
    pub height: u32,
    #[structopt(long)]
    pub emulator: Option<String>,
    #[structopt(name = "FILE")]
    pub file: Option<String>,
}

pub struct AppContext {
    pub args: CommandlineArgs,
    pub sdl: sdl2::Sdl,
    pub video: sdl2::VideoSubsystem,
    pub window: sdl2::video::Window,
    pub gl: sdl2::video::GLContext,
    pub event_pump: RefCell<sdl2::EventPump>,
    pub config_dir: PathBuf,
    pub data_dir: PathBuf,
    pub preferences: Preferences,
}

static mut APP_CONTEXT: Option<AppContext> = None;
static mut APPLICATION: Option<Py<App>> = None;

impl AppContext {
    pub fn init(
        args: CommandlineArgs,
        name: &str,
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
            .window(name, args.width, args.height)
            .position_centered()
            .resizable()
            .opengl()
            .allow_highdpi()
            .build()?;

        let event_pump = sdl.event_pump()?;
        let gl_context = window.gl_create_context()?;
        gl::load_with(|s| video.gl_get_proc_address(s) as _);

        let config_dir = config_dir.to_path_buf();
        let pref = config_dir.join("preferences.ron");
        let preferences = match Preferences::from_file(&pref) {
            Ok(p) => p,
            Err(e) => {
                info!("Error reading preferences: {:?}", e);
                info!("Using default preferences.");
                Preferences::default()
            }
        };

        unsafe {
            APP_CONTEXT = Some(AppContext {
                args: args,
                sdl: sdl,
                video: video,
                window: window,
                gl: gl_context,
                event_pump: RefCell::new(event_pump),
                config_dir: config_dir,
                data_dir: data_dir.to_path_buf(),
                preferences: preferences,
            });
        }
        Ok(())
    }

    pub fn get() -> &'static AppContext {
        unsafe { APP_CONTEXT.as_ref().unwrap() }
    }

    pub fn pref() -> &'static Preferences {
        &AppContext::get().preferences
    }

    pub fn pref_mut() -> &'static mut Preferences {
        unsafe { &mut APP_CONTEXT.as_mut().unwrap().preferences }
    }

    pub fn init_app(app: Py<App>) {
        unsafe {
            APPLICATION = Some(app);
        }
    }

    pub fn app() -> &'static Py<App> {
        unsafe { APPLICATION.as_ref().unwrap() }
    }

    pub fn emulator() -> String {
        if let Some(emulator) = &AppContext::get().args.emulator {
            emulator.clone()
        } else {
            AppContext::get().preferences.emulator.clone()
        }
    }
}
