use libc;
use nix::sys::termios;
use nix::sys::termios::SetArg;
use std::os::unix::io::RawFd;

const STDIN_FILENO: RawFd = libc::STDIN_FILENO;

pub struct UnixTerminalGuard {
    mode: termios::Termios,
}

impl UnixTerminalGuard {
    pub fn new() -> Self {
        UnixTerminalGuard {
            mode: termios::tcgetattr(STDIN_FILENO).unwrap(),
        }
    }

    pub fn restore_terminal(&self) {
        termios::tcsetattr(STDIN_FILENO, SetArg::TCSADRAIN, &self.mode).unwrap();
    }
}

impl Drop for UnixTerminalGuard {
    fn drop(&mut self) {
        self.restore_terminal();
    }
}
