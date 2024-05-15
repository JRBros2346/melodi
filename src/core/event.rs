use std::sync::{atomic::{AtomicBool, Ordering}, Arc};

use once_cell::sync::Lazy;

use crate::collections::Vect;

pub trait Sender: Send + Sync {
    fn fire(&self, event: impl Event) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            // If nothing is registered for the code, boot out.
            let (code, context) = event.get_id_and_context();
            if STATE[code as usize].is_empty() {
                return false;
            }
    
            for l in STATE[code as usize].iter() {
                if l.on_event(Some(self), event) {
                    // Message has been handled, do not send to other listeners.
                    return true;
                }
            }
    
            // Not found.
            false
        }
    }
}
impl Sender for () {
    fn fire(&self, event: impl Event) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            // If nothing is registered for the code, boot out.
            let (code, _) = event.get_id_and_context();
            if STATE[code as usize].is_empty() {
                return false;
            }
    
            for l in STATE[code as usize].iter() {
                if l.on_event(None, event) {
                    // Message has been handled, do not send to other listeners.
                    return true;
                }
            }
    
            // Not found.
            false
        }
    }
}

trait OnEvent {
    const N: u16 = 0;
    fn on_event(&self, sender: Option<&impl Sender>, event: EventContext) -> bool;
}

pub trait Listener: Send + Sync + OnEvent {
    fn register(&self, code: u16) -> bool {
        if !INIT.load(Ordering::Relaxed) {
            false
        } else {
            for l in STATE[code as usize].iter() {
                if &*l as *const _ as *const u8 == self as *const _ as *const u8 {
                    // TODO: warn
                    return false;
                }
            }
    
            // If at this point, no duplicate was found. Proceed with registration.
            STATE
                .get_mut(code as usize)
                .unwrap()
                .push(Arc::new(self));
            true
        }
    }
    fn unregister(&self, code: u16) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            // On nothing is registered for the code, boot out.
            if STATE[code as usize].is_empty() {
                // TODO: warn
                return false;
            }
    
            for i in 0..STATE[code as usize].len() {
                let l = STATE[code as usize][i];
                if &*l as *const _ as *const u8 == self as *const _ as *const u8 {
                    // Found one, remove it
                    STATE[code as usize].remove(i);
                    return true;
                }
            }
    
            // Not found.
            false
        }
    }
}

pub trait Event: Send + Sync {
    fn get_id_and_context(&self) -> (u16, EventContext);
}

// This should be more than enough codes...
const MAX_MESSAGE_CODES: usize = 65536;

/**
 * Event system internal state.
 */
static INIT: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<[Vect<Arc<dyn Listener>>; MAX_MESSAGE_CODES]> =
    Lazy::new(|| std::array::from_fn(|_| Vect::new()));



pub(crate) fn init() -> bool {
    if INIT.load(Ordering::Relaxed) {
        false
    } else {
        INIT.store(true, Ordering::Relaxed);
        true
    }
}
pub(crate) fn close() {}

// System internal event codes. Application should use codes beyond 255.
#[derive(PartialEq, Eq, Clone, Copy)]
pub enum SystemEvent {
    // Shuts the application down on the next frame.
    ApplicationQuit,

    // Keyboard key pressed.
    /* Context usage:
     * KeyPressed(key_code) => {
     *
     * }
     */
    KeyPressed(u16),

    // Keyboard key released.
    /* Context usage:
     * KeyReleased(key_code) => {
     *
     * }
     */
    KeyReleased(u16),

    // Mouse button pressed.
    /* Context usage:
     * ButtonPressed(button) => {
     *
     * }
     */
    ButtonPressed(u16),

    // Mouse button released.
    /* Context usage:
     * ButtonReleased(button) => {
     *
     * }
     */
    ButtonReleased(u16),

    // Mouse moved.
    /* Context usage:
     * MouseMoved(x, y) => {
     *
     * }
     */
    MouseMoved(u16, u16),

    // Mouse moved.
    /* Context usage:W
     * MouseWheel(z_delta) => {
     *
     * }
     */
    MouseWheel(u8),

    // Resized/resolution changed from the OS.
    /* Context usage:
     * Resized(width, height) => {
     *
     * }
     */
    Resized(u16, u16),
}

pub enum EventContext {
    // 256 bytes
    Unit,

    I128([i128; 2]),
    U128([u128; 2]),

    I64([i64; 4]),
    U64([u64; 4]),
    F64([f64; 4]),

    I32([i32; 8]),
    U32([u32; 8]),
    F32([f32; 8]),
    C([char; 8]),

    I16([i16; 16]),
    U16([u16; 16]),

    I8([i8; 32]),
    U8([u8; 32]),
}

impl Event for SystemEvent {
    fn get_id_and_context(&self) -> (u16, EventContext) {
        match self {
            Self::ApplicationQuit => (0x01, EventContext::Unit),
            Self::KeyPressed(key_code) => (0x02, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *key_code;
                out
            })),
            Self::KeyReleased(key_code) => (0x03, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *key_code;
                out
            })),
            Self::ButtonPressed(button) => (0x04, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *button;
                out
            })),
            Self::ButtonReleased(button) => (0x05, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *button;
                out
            })),
            Self::MouseMoved(x, y) => (0x06, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *x;
                out[1] = *y;
                out
            })),
            Self::MouseWheel(z_delta) => (0x07, EventContext::U8({
                let mut out = [0; 32];
                out[0] = *z_delta;
                out
            })),
            Self::Resized(width, height) => (0x08, EventContext::U16({
                let mut out = [0; 16];
                out[0] = *width;
                out[1] = *height;
                out
            })),
        }
    }
}