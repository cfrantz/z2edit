#![recursion_limit = "256"]
#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate directories;
extern crate env_logger;
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
extern crate typetag;
extern crate whoami;

pub mod errors;
pub mod gui;
pub mod nes;
pub mod util;
pub mod zelda2;

use directories::ProjectDirs;
use log::LevelFilter;
use pyo3::prelude::*;
use std::fs;
use std::io;
use structopt::StructOpt;
use util::build;
use util::TerminalGuard;

use crate::errors::*;
use crate::gui::app::App;
use crate::gui::app_context::{AppContext, CommandlineArgs};
use crate::util::pyaddress::Address;
use crate::zelda2::config::Config as Zelda2Config;
use crate::zelda2::config::PyConfig;
use crate::zelda2::project::Project;
use crate::zelda2::text_encoding::python::Text;

fn setup_pythonsite(py: Python) -> Result<()> {
    let mut pydir = AppContext::installation_dir()?;
    pydir.push("python");
    info!("Configuring python sidedir: {:?}", pydir);
    let module = PyModule::import(py, "site")?;
    if let Some(dir) = pydir.to_str() {
        module.call("addsitedir", (dir,), None)?;
        Ok(())
    } else {
        Err(ErrorKind::NotFound(format!("Could not convert directory name: {:?}", pydir)).into())
    }
}

fn run(py: Python) -> Result<()> {
    let args = CommandlineArgs::from_args();

    setup_pythonsite(py)?;
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
    module.add_class::<Address>()?;
    module.add_class::<Project>()?;
    module.add_class::<Text>()?;
    module.setattr("app", app)?;
    module.setattr("version", build::version())?;
    let pycfg = Py::new(py, PyConfig {})?;
    module.setattr("config", pycfg)?;

    if let Some(file) = &AppContext::get().args.file {
        app.borrow(py).load_project(py, file)?;
    }

    App::run(app, py);
    Ok(())
}

fn main() {
    let mut builder = env_logger::Builder::from_default_env();
    builder.filter(None, LevelFilter::Info).init();
    info!("Z2Edit version {}", build::version());

    let _mode = TerminalGuard::new();
    Python::with_gil(|py| {
        run(py).unwrap();
    });
}
