use std::sync::{
    atomic::{AtomicBool, AtomicI16, Ordering},
    Mutex,
};

use once_cell::sync::Lazy;
use strum::EnumCount;

use super::event;
use event::{ButtonPress, ButtonRelease, KeyPress, KeyRelease, MouseMove, MouseWheel};

#[repr(u8)]
#[derive(Clone, Copy, strum::EnumCount)]
pub enum Button {
    Left,
    Middle,
    Right,
}

#[repr(u8)]
#[derive(Clone, Copy, strum::FromRepr)]
pub enum Key {
    BackSpace = 0x08,
    Enter = 0x0D,
    Tab = 0x09,
    Shift = 0x10,
    Ctrl = 0x11,

    Pause = 0x13,
    CapsLock = 0x14,

    Esc = 0x1B,

    Convert = 0x1C,
    NonConvert = 0x1D,
    Accept = 0x1E,
    ModeChange = 0x1F,

    Space = 0x20,
    Prior = 0x21,
    Next = 0x22,
    End = 0x23,
    Home = 0x24,
    Left = 0x25,
    Up = 0x26,
    Right = 0x27,
    Down = 0x28,
    Select = 0x29,
    Print = 0x2A,
    Execute = 0x2B,
    Snapshot = 0x2C,
    Insert = 0x2D,
    Delete = 0x2E,
    Help = 0x2F,

    A = 0x41,
    B = 0x42,
    C = 0x43,
    D = 0x44,
    E = 0x45,
    F = 0x46,
    G = 0x47,
    H = 0x48,
    I = 0x49,
    J = 0x4A,
    K = 0x4B,
    L = 0x4C,
    M = 0x4D,
    N = 0x4E,
    O = 0x4F,
    P = 0x50,
    Q = 0x51,
    R = 0x52,
    S = 0x53,
    T = 0x54,
    U = 0x55,
    V = 0x56,
    W = 0x57,
    X = 0x58,
    Y = 0x59,
    Z = 0x5A,

    LeftWin = 0x5B,
    RightWin = 0x5C,
    Apps = 0x5D,

    Sleep = 0x5F,

    NumPad0 = 0x60,
    NumPad1 = 0x61,
    NumPad2 = 0x62,
    NumPad3 = 0x63,
    NumPad4 = 0x64,
    NumPad5 = 0x65,
    NumPad6 = 0x66,
    NumPad7 = 0x67,
    NumPad8 = 0x68,
    NumPad9 = 0x69,
    Multiply = 0x6A,
    Add = 0x6B,
    Separator = 0x6C,
    Subtract = 0x6D,
    Decimal = 0x6E,
    Divide = 0x6F,
    F1 = 0x70,
    F2 = 0x71,
    F3 = 0x72,
    F4 = 0x73,
    F5 = 0x74,
    F6 = 0x75,
    F7 = 0x76,
    F8 = 0x77,
    F9 = 0x78,
    F10 = 0x79,
    F11 = 0x7A,
    F12 = 0x7B,
    F13 = 0x7C,
    F14 = 0x7D,
    F15 = 0x7E,
    F16 = 0x7F,
    F17 = 0x80,
    F18 = 0x81,
    F19 = 0x82,
    F20 = 0x83,
    F21 = 0x84,
    F22 = 0x85,
    F23 = 0x86,
    F24 = 0x87,

    NumLock = 0x90,
    Scroll = 0x91,

    NumPadEqual = 0x92,

    LeftShift = 0xA0,
    RightShift = 0xA1,
    LeftCtrl = 0xA2,
    RightCtrl = 0xA3,
    LeftMenu = 0xA4,
    RightMenu = 0xA5,

    Semicolon = 0xBA,
    Plus = 0xBB,
    Comma = 0xBC,
    Minus = 0xBD,
    Period = 0xBE,
    Slash = 0xBF,
    Grave = 0xC0,
}

type KeyboardState = Lazy<Mutex<[bool; u8::BITS as usize]>>;
type MouseState = ((AtomicI16, AtomicI16), Lazy<Mutex<[bool; Button::COUNT]>>);

static INIT: AtomicBool = AtomicBool::new(false);
static STATE: (
    KeyboardState,
    KeyboardState,
    MouseState,
    MouseState,
) = (
    Lazy::new(|| Mutex::new([false; u8::BITS as usize])),
    Lazy::new(|| Mutex::new([false; u8::BITS as usize])),
    (
        (AtomicI16::new(0), AtomicI16::new(0)),
        Lazy::new(|| Mutex::new([false; Button::COUNT])),
    ),
    (
        (AtomicI16::new(0), AtomicI16::new(0)),
        Lazy::new(|| Mutex::new([false; Button::COUNT])),
    ),
);

pub(crate) fn init() {
    *STATE.0.lock().unwrap() = [false; u8::BITS as usize];
    *STATE.1.lock().unwrap() = [false; u8::BITS as usize];
    STATE.2 .0 .0.store(0, Ordering::Relaxed);
    STATE.2 .0 .1.store(0, Ordering::Relaxed);
    *STATE.2 .1.lock().unwrap() = [false; Button::COUNT];
    STATE.3 .0 .0.store(0, Ordering::Relaxed);
    STATE.3 .0 .1.store(0, Ordering::Relaxed);
    *STATE.3 .1.lock().unwrap() = [false; Button::COUNT];
    INIT.store(true, Ordering::Relaxed);
    crate::info!("Input Subsystem Initialized");
}
pub(crate) fn close() {
    // TODO: Add shutdown routines when needed.
    INIT.store(false, Ordering::Relaxed);
}
pub(crate) fn update(delta_timee: f64) {
    if INIT.load(Ordering::Relaxed) {
        let mut keyboard_then = STATE.0.lock().unwrap();
        let keyboard_now = STATE.1.lock().unwrap();
        let mut mouse_then = STATE.2 .1.lock().unwrap();
        let mouse_now = STATE.3 .1.lock().unwrap();

        // Copy Current State to previous State
        *keyboard_then = *keyboard_now;
        STATE
            .2
             .0
             .0
            .store(STATE.3 .0 .0.load(Ordering::Relaxed), Ordering::Relaxed);
        STATE
            .2
             .0
             .1
            .store(STATE.3 .0 .1.load(Ordering::Relaxed), Ordering::Relaxed);
        *mouse_then = *mouse_now;
    }
}

// Keyboard Input
pub fn is_key_up(key: Key) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        true
    } else {
        !STATE.1.lock().unwrap()[key as usize]
    }
}
pub fn is_key_down(key: Key) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        false
    } else {
        STATE.1.lock().unwrap()[key as usize]
    }
}
pub fn was_key_up(key: Key) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        true
    } else {
        !STATE.0.lock().unwrap()[key as usize]
    }
}
pub fn was_key_down(key: Key) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        false
    } else {
        STATE.0.lock().unwrap()[key as usize]
    }
}

#[allow(dead_code)]
pub(crate) fn process_key(key: Key, press: bool) {
    let mut lock = STATE.1.lock().unwrap();
    // Only handle this if the state actually changes
    if lock[key as usize] != press {
        // Update internal state
        lock[key as usize] = press;

        // Fire off an event
        if press {
            event::fire(KeyPress(key));
        } else {
            event::fire(KeyRelease(key));
        }
    }
}

// Mouse Input
pub fn is_button_up(button: Button) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        true
    } else {
        STATE.1.lock().unwrap()[button as usize]
    }
}
pub fn is_button_down(button: Button) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        false
    } else {
        STATE.1.lock().unwrap()[button as usize]
    }
}
pub fn was_button_up(button: Button) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        true
    } else {
        !STATE.0.lock().unwrap()[button as usize]
    }
}
pub fn was_button_down(button: Button) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        false
    } else {
        STATE.0.lock().unwrap()[button as usize]
    }
}
pub fn get_mouse_now() -> (i16, i16) {
    if !INIT.load(Ordering::Relaxed) {
        (0, 0)
    } else {
        (
            STATE.3 .0 .0.load(Ordering::Relaxed),
            STATE.3 .0 .1.load(Ordering::Relaxed),
        )
    }
}
pub fn get_mouse_then() -> (i16, i16) {
    if !INIT.load(Ordering::Relaxed) {
        (0, 0)
    } else {
        (
            STATE.2 .0 .0.load(Ordering::Relaxed),
            STATE.2 .0 .1.load(Ordering::Relaxed),
        )
    }
}

#[allow(dead_code)]
pub(crate) fn process_button(button: Button, press: bool) {
    let mut lock = STATE.3 .1.lock().unwrap();
    // Only handle this if the state actually changes
    if lock[button as usize] != press {
        // Update internal state
        lock[button as usize] = press;

        // Fire off an event
        if press {
            event::fire(ButtonPress(button));
        } else {
            event::fire(ButtonRelease(button));
        }
    }
}
#[allow(dead_code)]
pub(crate) fn process_mouse_move(x: i16, y: i16) {
    // Only process if actually different
    if (
        STATE.3 .0 .0.load(Ordering::Relaxed),
        STATE.3 .0 .1.load(Ordering::Relaxed),
    ) != (x, y)
    {
        // NOTE: Enable this if debugging.
        crate::debug!("Mouse Position: {x} {y}");

        // Update internal state
        STATE.3 .0 .0.store(x, Ordering::Relaxed);
        STATE.3 .0 .1.store(y, Ordering::Relaxed);

        // Fire event
        event::fire(MouseMove { x, y });
    }
}
#[allow(dead_code)]
pub(crate) fn process_mouse_wheel(z_delta: i8) {
    // NOTE: No internal state to update
    // Fire the event.
    event::fire(MouseWheel(z_delta));
}
