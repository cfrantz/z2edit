pub mod pyaddress;
pub mod pyexec;
pub mod terminal;
pub mod time;
pub mod undo;

use terminal::UnixTerminalGuard;

pub type TerminalGuard = UnixTerminalGuard;
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
