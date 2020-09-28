#[macro_use]
extern crate error_chain;
extern crate structopt;
extern crate directories;
extern crate gl;
extern crate sdl2;
#[macro_use]
extern crate log;
extern crate simplelog;
extern crate rustyline;

pub mod gui;
pub mod errors;
pub mod util;


use std::fs;
use std::io;
use crate::errors::*;
use simplelog::*;
use directories::ProjectDirs;
use gui::app::App;
use gui::app_context::AppContext;
use structopt::StructOpt;
use util::TerminalGuard;

#[derive(StructOpt, Debug)]
#[structopt(name = "z2edit")]
struct Opt {
    #[structopt(short, long, default_value = "1280")]
    width: u32,
    #[structopt(short, long, default_value = "720")]
    height: u32,
}

fn run() -> Result<()> {
    let opt = Opt::from_args();
    CombinedLogger::init(vec![TermLogger::new(
            LevelFilter::Info,
            Config::default(),
            TerminalMode::Mixed).unwrap()]
    ).unwrap();

    let dirs = ProjectDirs::from("org", "CF207", "Z2Edit").ok_or_else(
        || io::Error::new(io::ErrorKind::NotFound, "Could not find home directory"))?;

    let config = dirs.config_dir();
    info!("Config dir: {}", config.display());
    fs::create_dir_all(config).map_err(|e| ErrorKind::Io(e))?;

    let data = dirs.data_dir();
    info!("Data dir: {}", data.display());
    fs::create_dir_all(data).map_err(|e| ErrorKind::Io(e))?;

    AppContext::init("Z2Edit", opt.width, opt.height, config, data)?;
    let _mode = TerminalGuard::new();
    let mut app = App::new();
    app.run();
    Ok(())
}

fn main() {
    run().unwrap();
}
