use imgui::Key;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::controller::Controller;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InputMap {
    pub controller: Vec<HashMap<KeyRef, ControllerRef>>,
    pub command: HashMap<KeyRef, CommandKey>,
}

impl Default for InputMap {
    fn default() -> Self {
        InputMap {
            controller: vec![
                HashMap::from([
                    (KeyRef::LeftAlt, ControllerRef::A),
                    (KeyRef::LeftCtrl, ControllerRef::B),
                    (KeyRef::LeftShift, ControllerRef::Select),
                    (KeyRef::Tab, ControllerRef::Start),
                    (KeyRef::UpArrow, ControllerRef::Up),
                    (KeyRef::DownArrow, ControllerRef::Down),
                    (KeyRef::LeftArrow, ControllerRef::Left),
                    (KeyRef::RightArrow, ControllerRef::Right),
                ]),
                HashMap::from([(KeyRef::F12, ControllerRef::UpPlusA)]),
            ],
            command: HashMap::from([
                (KeyRef::F11, CommandKey::SystemReset),
                (KeyRef::Pause, CommandKey::SystemPause),
                (KeyRef::Backslash, CommandKey::SystemFrameStep),
                (KeyRef::F5, CommandKey::SystemSaveState),
                (KeyRef::F7, CommandKey::SystemRestoreState),
                (KeyRef::Alpha0, CommandKey::SelectState0),
                (KeyRef::Alpha1, CommandKey::SelectState1),
                (KeyRef::Alpha2, CommandKey::SelectState2),
                (KeyRef::Alpha3, CommandKey::SelectState3),
                (KeyRef::Alpha4, CommandKey::SelectState4),
                (KeyRef::Alpha5, CommandKey::SelectState5),
                (KeyRef::Alpha6, CommandKey::SelectState6),
                (KeyRef::Alpha7, CommandKey::SelectState7),
                (KeyRef::Alpha8, CommandKey::SelectState8),
                (KeyRef::Alpha9, CommandKey::SelectState9),
                // I think imgui-sdl-support has a bug and incorrectly maps
                // the number keys to the numeric keypad keys.
                (KeyRef::Keypad0, CommandKey::SelectState0),
                (KeyRef::Keypad1, CommandKey::SelectState1),
                (KeyRef::Keypad2, CommandKey::SelectState2),
                (KeyRef::Keypad3, CommandKey::SelectState3),
                (KeyRef::Keypad4, CommandKey::SelectState4),
                (KeyRef::Keypad5, CommandKey::SelectState5),
                (KeyRef::Keypad6, CommandKey::SelectState6),
                (KeyRef::Keypad7, CommandKey::SelectState7),
                (KeyRef::Keypad8, CommandKey::SelectState8),
                (KeyRef::Keypad9, CommandKey::SelectState9),
            ]),
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum CommandKey {
    SystemQuit,
    SystemReset,
    SystemPause,
    SystemFrameStep,
    SystemSaveState,
    SystemRestoreState,

    SelectState0,
    SelectState1,
    SelectState2,
    SelectState3,
    SelectState4,
    SelectState5,
    SelectState6,
    SelectState7,
    SelectState8,
    SelectState9,
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum ControllerRef {
    A = Controller::BUTTON_A,
    B = Controller::BUTTON_B,
    Select = Controller::BUTTON_SELECT,
    Start = Controller::BUTTON_START,
    Up = Controller::BUTTON_UP,
    Down = Controller::BUTTON_DOWN,
    Left = Controller::BUTTON_LEFT,
    Right = Controller::BUTTON_RIGHT,
    UpPlusA = Controller::BUTTON_UP | Controller::BUTTON_A,
}

impl From<ControllerRef> for u8 {
    fn from(c: ControllerRef) -> u8 {
        c as u8
    }
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[non_exhaustive]
pub enum KeyRef {
    Tab = Key::Tab as u32,
    LeftArrow = Key::LeftArrow as u32,
    RightArrow = Key::RightArrow as u32,
    UpArrow = Key::UpArrow as u32,
    DownArrow = Key::DownArrow as u32,
    PageUp = Key::PageUp as u32,
    PageDown = Key::PageDown as u32,
    Home = Key::Home as u32,
    End = Key::End as u32,
    Insert = Key::Insert as u32,
    Delete = Key::Delete as u32,
    Backspace = Key::Backspace as u32,
    Space = Key::Space as u32,
    Enter = Key::Enter as u32,
    Escape = Key::Escape as u32,
    LeftCtrl = Key::LeftCtrl as u32,
    LeftShift = Key::LeftShift as u32,
    LeftAlt = Key::LeftAlt as u32,
    LeftSuper = Key::LeftSuper as u32,
    RightCtrl = Key::RightCtrl as u32,
    RightShift = Key::RightShift as u32,
    RightAlt = Key::RightAlt as u32,
    RightSuper = Key::RightSuper as u32,
    Menu = Key::Menu as u32,
    Alpha0 = Key::Alpha0 as u32,
    Alpha1 = Key::Alpha1 as u32,
    Alpha2 = Key::Alpha2 as u32,
    Alpha3 = Key::Alpha3 as u32,
    Alpha4 = Key::Alpha4 as u32,
    Alpha5 = Key::Alpha5 as u32,
    Alpha6 = Key::Alpha6 as u32,
    Alpha7 = Key::Alpha7 as u32,
    Alpha8 = Key::Alpha8 as u32,
    Alpha9 = Key::Alpha9 as u32,
    A = Key::A as u32,
    B = Key::B as u32,
    C = Key::C as u32,
    D = Key::D as u32,
    E = Key::E as u32,
    F = Key::F as u32,
    G = Key::G as u32,
    H = Key::H as u32,
    I = Key::I as u32,
    J = Key::J as u32,
    K = Key::K as u32,
    L = Key::L as u32,
    M = Key::M as u32,
    N = Key::N as u32,
    O = Key::O as u32,
    P = Key::P as u32,
    Q = Key::Q as u32,
    R = Key::R as u32,
    S = Key::S as u32,
    T = Key::T as u32,
    U = Key::U as u32,
    V = Key::V as u32,
    W = Key::W as u32,
    X = Key::X as u32,
    Y = Key::Y as u32,
    Z = Key::Z as u32,
    F1 = Key::F1 as u32,
    F2 = Key::F2 as u32,
    F3 = Key::F3 as u32,
    F4 = Key::F4 as u32,
    F5 = Key::F5 as u32,
    F6 = Key::F6 as u32,
    F7 = Key::F7 as u32,
    F8 = Key::F8 as u32,
    F9 = Key::F9 as u32,
    F10 = Key::F10 as u32,
    F11 = Key::F11 as u32,
    F12 = Key::F12 as u32,
    Apostrophe = Key::Apostrophe as u32,
    Comma = Key::Comma as u32,
    Minus = Key::Minus as u32,
    Period = Key::Period as u32,
    Slash = Key::Slash as u32,
    Semicolon = Key::Semicolon as u32,
    Equal = Key::Equal as u32,
    LeftBracket = Key::LeftBracket as u32,
    Backslash = Key::Backslash as u32,
    RightBracket = Key::RightBracket as u32,
    GraveAccent = Key::GraveAccent as u32,
    CapsLock = Key::CapsLock as u32,
    ScrollLock = Key::ScrollLock as u32,
    NumLock = Key::NumLock as u32,
    PrintScreen = Key::PrintScreen as u32,
    Pause = Key::Pause as u32,
    Keypad0 = Key::Keypad0 as u32,
    Keypad1 = Key::Keypad1 as u32,
    Keypad2 = Key::Keypad2 as u32,
    Keypad3 = Key::Keypad3 as u32,
    Keypad4 = Key::Keypad4 as u32,
    Keypad5 = Key::Keypad5 as u32,
    Keypad6 = Key::Keypad6 as u32,
    Keypad7 = Key::Keypad7 as u32,
    Keypad8 = Key::Keypad8 as u32,
    Keypad9 = Key::Keypad9 as u32,
    KeypadDecimal = Key::KeypadDecimal as u32,
    KeypadDivide = Key::KeypadDivide as u32,
    KeypadMultiply = Key::KeypadMultiply as u32,
    KeypadSubtract = Key::KeypadSubtract as u32,
    KeypadAdd = Key::KeypadAdd as u32,
    KeypadEnter = Key::KeypadEnter as u32,
    KeypadEqual = Key::KeypadEqual as u32,
    GamepadStart = Key::GamepadStart as u32,
    GamepadBack = Key::GamepadBack as u32,
    GamepadFaceLeft = Key::GamepadFaceLeft as u32,
    GamepadFaceRight = Key::GamepadFaceRight as u32,
    GamepadFaceUp = Key::GamepadFaceUp as u32,
    GamepadFaceDown = Key::GamepadFaceDown as u32,
    GamepadDpadLeft = Key::GamepadDpadLeft as u32,
    GamepadDpadRight = Key::GamepadDpadRight as u32,
    GamepadDpadUp = Key::GamepadDpadUp as u32,
    GamepadDpadDown = Key::GamepadDpadDown as u32,
    GamepadL1 = Key::GamepadL1 as u32,
    GamepadR1 = Key::GamepadR1 as u32,
    GamepadL2 = Key::GamepadL2 as u32,
    GamepadR2 = Key::GamepadR2 as u32,
    GamepadL3 = Key::GamepadL3 as u32,
    GamepadR3 = Key::GamepadR3 as u32,
    GamepadLStickLeft = Key::GamepadLStickLeft as u32,
    GamepadLStickRight = Key::GamepadLStickRight as u32,
    GamepadLStickUp = Key::GamepadLStickUp as u32,
    GamepadLStickDown = Key::GamepadLStickDown as u32,
    GamepadRStickLeft = Key::GamepadRStickLeft as u32,
    GamepadRStickRight = Key::GamepadRStickRight as u32,
    GamepadRStickUp = Key::GamepadRStickUp as u32,
    GamepadRStickDown = Key::GamepadRStickDown as u32,
    MouseLeft = Key::MouseLeft as u32,
    MouseRight = Key::MouseRight as u32,
    MouseMiddle = Key::MouseMiddle as u32,
    MouseX1 = Key::MouseX1 as u32,
    MouseX2 = Key::MouseX2 as u32,
    MouseWheelX = Key::MouseWheelX as u32,
    MouseWheelY = Key::MouseWheelY as u32,
    ReservedForModCtrl = Key::ReservedForModCtrl as u32,
    ReservedForModShift = Key::ReservedForModShift as u32,
    ReservedForModAlt = Key::ReservedForModAlt as u32,
    ReservedForModSuper = Key::ReservedForModSuper as u32,
}

impl From<KeyRef> for Key {
    fn from(key: KeyRef) -> Key {
        // SAFETY: KeyRef has identical representation to imgui::Key.
        unsafe { std::mem::transmute(key) }
    }
}
