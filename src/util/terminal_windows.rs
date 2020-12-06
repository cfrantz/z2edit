pub struct TerminalGuard;

impl TerminalGuard {
    pub fn new() -> Self {
        TerminalGuard {}
    }

    pub fn restore_terminal(&self) {}
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        self.restore_terminal();
    }
}
