#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate directories;
extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate smart_default;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate dict_derive;
extern crate pyo3;
extern crate rustyline;
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
use crate::util::pyexec::PythonExecutor;
use crate::zelda2::config::Config as Zelda2Config;
use crate::zelda2::project::Project;

fn run(py: Python) -> Result<()> {
    let args = CommandlineArgs::from_args();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();

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
    let mut executor = PythonExecutor::new(py)?;

    let module = PyModule::new(py, "z2edit")?;
    app.borrow(py).pythonize(py, module)?;
    module.add_class::<PyAddress>()?;
    module.add_class::<Project>()?;
    module.setattr("app", app)?;

    let sys = PyModule::import(py, "sys")?;
    let modules = sys.get("modules")?;
    modules.set_item("z2edit", module)?;

    let assembler = PyModule::from_code(
        py,
        include_str!("../python/assembler.py"),
        "assembler.py",
        "assembler",
    )?;
    modules.set_item("assembler", assembler)?;

    let debug = PyModule::from_code(py, include_str!("../python/debug.py"), "debug.py", "debug")?;
    modules.set_item("debug", debug)?;

    App::run(app, py, &mut executor);
    Ok(())
}

fn main() {
    let _mode = TerminalGuard::new();
    Python::with_gil(|py| {
        run(py).unwrap();
    });
}
