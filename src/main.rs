#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate directories;
extern crate gl;
extern crate indexmap;
extern crate pathdiff;
extern crate sdl2;
#[macro_use]
extern crate smart_default;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate dict_derive;
extern crate pyo3;
extern crate rand;
extern crate rustyline;
extern crate sha2;
extern crate shellwords;
extern crate simplelog;
extern crate typetag;
extern crate whoami;

pub mod errors;
pub mod gui;
pub mod nes;
pub mod util;
pub mod zelda2;

use directories::ProjectDirs;
use pyo3::prelude::*;
use simplelog::*;
use std::fs;
use std::io;
use structopt::StructOpt;
use util::TerminalGuard;

use crate::errors::*;
use crate::gui::app::App;
use crate::gui::app_context::{AppContext, CommandlineArgs};
use crate::util::pyaddress::PyAddress;
use crate::zelda2::config::Config as Zelda2Config;
use crate::zelda2::config::PyConfig;
use crate::zelda2::project::Project;
use crate::zelda2::text_encoding::python::Text;

fn setup_pythonpath() {
    if let Some(path) = std::env::var_os("PYTHONPATH") {
        let mut paths = std::env::split_paths(&path).collect::<Vec<_>>();
        let mut pydir = AppContext::installation_dir().expect("pydir");
        pydir.push("python");
        paths.push(pydir);
        let new_path = std::env::join_paths(paths).expect("pythonpath");
        info!("PYTHONPATH={:?}", new_path);
        std::env::set_var("PYTHONPATH", &new_path);
    }
}

fn run(py: Python) -> Result<()> {
    let args = CommandlineArgs::from_args();

    // Force evaluation of the config early.
    let _ = Zelda2Config::get("vanilla");

    let dirs = ProjectDirs::from("org", "CF207", "Z2Edit")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;

    let config = dirs.config_dir();
    info!("Config dir: {}", config.display());
    fs::create_dir_all(config).map_err(|e| ErrorKind::Io(e))?;

    let data = dirs.data_dir();
    info!("Data dir: {}", data.display());
    fs::create_dir_all(data).map_err(|e| ErrorKind::Io(e))?;

    AppContext::init(args, "Z2Edit", config, data)?;
    AppContext::init_app(Py::new(py, App::new(py)?)?);
    let app = AppContext::app();

    let module = PyModule::import(py, "z2edit")?;
    app.borrow(py).pythonize(py, module)?;
    module.add_class::<PyAddress>()?;
    module.add_class::<Project>()?;
    module.add_class::<Text>()?;
    module.setattr("app", app)?;
    let pycfg = Py::new(py, PyConfig {})?;
    module.setattr("config", pycfg)?;

    if let Some(file) = &AppContext::get().args.file {
        app.borrow(py).load_project(py, file)?;
    }

    App::run(app, py);
    Ok(())
}

fn main() {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();

    setup_pythonpath();
    let _mode = TerminalGuard::new();
    Python::with_gil(|py| {
        run(py).unwrap();
    });
}
