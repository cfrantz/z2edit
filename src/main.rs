#[macro_use]
extern crate error_chain;
extern crate chrono;
extern crate directories;
extern crate gl;
extern crate sdl2;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate pyo3;
extern crate rustyline;
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
use crate::gui::app_context::AppContext;
use crate::util::pyaddress::PyAddress;
use crate::util::pyexec::PythonExecutor;
use crate::zelda2::project::Project;

use crate::zelda2::enemyattr::config::Config as Ecfg;

#[derive(StructOpt, Debug)]
#[structopt(name = "z2edit")]
struct Opt {
    #[structopt(short, long, default_value = "1280")]
    width: u32,
    #[structopt(short, long, default_value = "720")]
    height: u32,
}

fn run(py: Python) -> Result<()> {
    let opt = Opt::from_args();
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )
    .unwrap()])
    .unwrap();

    let dirs = ProjectDirs::from("org", "CF207", "Z2Edit")
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;

    let config = dirs.config_dir();
    info!("Config dir: {}", config.display());
    fs::create_dir_all(config).map_err(|e| ErrorKind::Io(e))?;

    let data = dirs.data_dir();
    info!("Data dir: {}", data.display());
    fs::create_dir_all(data).map_err(|e| ErrorKind::Io(e))?;

    AppContext::init("Z2Edit", opt.width, opt.height, config, data)?;
    let app = PyCell::new(py, App::new(py)?)?;
    let mut executor = PythonExecutor::new(py)?;

    let module = PyModule::new(py, "z2edit")?;
    app.borrow().pythonize(py, module)?;
    module.add_class::<PyAddress>()?;
    module.add_class::<Project>()?;
    module.setattr("app", app)?;

    let sys = PyModule::import(py, "sys")?;
    let modules = sys.get("modules")?;
    modules.set_item("z2edit", module)?;

    App::run(&app, py, &mut executor);
    Ok(())
}

fn main() {
    let _mode = TerminalGuard::new();
    Python::with_gil(|py| {
        run(py).unwrap();
    });
}
