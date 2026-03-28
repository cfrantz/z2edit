# About `rust/z2edit/src/gui`

This directory contains the core user interface components for the `z2edit` application. These modules provide the high-level application framework and generic dialogs, whereas game-specific editors are located in the `zelda2/` subdirectory.

### Core GUI Framework
- **`project.rs`**: The primary dashboard for a romhack project, coordinating all active editor windows and state.
- **`wizard.rs`**: A guided workflow for initializing new romhack projects.
- **`preferences.rs`**: Centralized UI for configuring application-wide settings.

### Shared UI Components
- **`widgets.rs`**: Reusable custom ImGUI widgets tailored for ROM editing.
- **`util.rs`**: GUI-specific layout and rendering helpers.
- **`file_dialog.rs`**: Cross-platform file selection wrappers.
- **`error_dialog.rs`**: Standardized interface for displaying error messages to the user.
- **`visibility.rs`**: Window management logic.
