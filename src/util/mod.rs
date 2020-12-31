pub mod pyaddress;
pub mod pyexec;
pub mod relative_path;
pub mod time;
pub mod undo;

#[cfg_attr(target_os = "linux", path = "terminal_unix.rs")]
#[cfg_attr(target_os = "windows", path = "terminal_windows.rs")]
pub mod terminal;

pub use terminal::TerminalGuard;

pub use time::UTime;

pub fn clamp<T: PartialOrd>(a: T, mn: T, mx: T) -> T {
    if a < mn {
        mn
    } else if a > mx {
        mx
    } else {
        a
    }
}
