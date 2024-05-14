use std::sync::atomic::{AtomicBool, Ordering};

use once_cell::sync::Lazy;

use crate::collections::Vect;

pub trait Sender: Send + Sync {}
pub trait Listener: Send + Sync {}
pub trait EventCode: Send + Sync {
    fn code(&self) -> u32;
}

// This should be more than enough codes...
const MAX_MESSAGE_CODES: usize = 65536;

/**
 * Event system internal state.
 */
static INIT: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<[Vect<(&'static dyn Listener, &'static OnEvent)>; MAX_MESSAGE_CODES]> = Lazy::new(|| std::array::from_fn(|_| Vect::new()));

pub enum EventContext {
    // 256 bytes
    I128([i128; 2]),
    U128([u128; 2]),

    I64([i64; 4]),
    U64([u64; 4]),
    F64([f64; 4]),

    I32([i32; 8]),
    U32([u32; 8]),
    F32([f32; 8]),

    I16([i16; 16]),
    U16([u16; 16]),
    C([char; 16]),

    I8([i8; 32]),
    U8([u8; 32]),
}

// Should return true if handled.
type OnEvent = dyn Fn(&'static dyn EventCode, Option<&'static dyn Sender>, &'static dyn Listener, EventContext) -> bool + Send + Sync + 'static;

pub(crate) fn init() -> bool {
    if INIT.load(Ordering::Relaxed) {
        false
    } else {
        INIT.store(true, Ordering::Relaxed);
        true
    }
}
pub(crate) fn close() {

}

/**
 * Register to listen for when events are sent with the provided code. Events with duplicate
 * listener/callback combos will not be registered again and will cause this to return FALSE.
 * @param code The event code to listen for.
 * @param listener A pointer to a listener instance. Can be 0/NULL.
 * @param on_event The callback function pointer to be invoked when the event code is fired.
 * @returns TRUE if the event is successfully registered; otherwise false.
 */
pub fn register(code: impl EventCode, listener: &impl Listener, on_event: &OnEvent) -> bool {
    if !INIT.load(Ordering::Relaxed) {
        false
    } else {
        for e in STATE[code.code() as usize].iter() {
            if listener as *const _ as *const u8 == e.0 as *const _ as *const u8 {
                // TODO: warn
                return false;
            }
        }
    
        // If at this point, no duplicate was found. Proceed with registration.
        STATE.get_mut(code.code() as usize).unwrap().push((listener, on_event));
        true
    }
}

/**
 * Unregister from listening for when events are sent with the provided code. If no matching
 * registration is found, this function returns FALSE.
 * @param code The event code to stop listening for.
 * @param listener A pointer to a listener instance. Can be 0/NULL.
 * @param on_event The callback function pointer to be unregistered.
 * @returns TRUE if the event is successfully unregistered; otherwise false.
 */
pub fn unregister(code: impl EventCode, listener: &impl Listener, on_event: &OnEvent) -> bool {
    if INIT.load(Ordering::Relaxed) {
        false
    } else {
        // On nothing is registered for the code, boot out.
        if STATE[code.code() as usize].is_empty() {
            // TODO: warn
            return false;
        }

        for i in 0..STATE[code.code() as usize].len() {
            let e = STATE[code.code() as usize][i];
            if e.0 as *const _ as *const u8 == listener as *const _ as *const u8 && e.1 as *const _ as *const u8 == on_event as *const _ as *const u8 {
                // Found one, remove it
                STATE[code.code() as usize].remove(i);
                return true;
            }
        }
    
        // Not found.
        false
    }

}

/**
 * Fires an event to listeners of the given code. If an event handler returns 
 * TRUE, the event is considered handled and is not passed on to any more listeners.
 * @param code The event code to fire.
 * @param sender A pointer to the sender. Can be 0/NULL.
 * @param data The event data.
 * @returns TRUE if handled, otherwise FALSE.
 */
pub fn fire(code: impl EventCode, sender: Option<&dyn Sender>, context: EventContext) -> bool {
    if INIT.load(Ordering::Relaxed) {
        false
    } else {
        // If nothing is registered for the code, boot out.
        if STATE[code.code() as usize].is_empty() {
            return false;
        }

        for e in STATE[code.code() as usize] {
            if e.1(&code, sender, e.0, context) {
                // Message has been handled, do not send to other listeners.
                return true;
            }
        }
    
        // Not found.
        false
    }

}

// System internal event codes. Application should use codes beyond 255.
#[repr(u32)]
#[derive(PartialEq, Eq)]
pub enum SystemEventCode {
    // Shuts the application down on the next frame.
    ApplicationQuit = 0x01,

    // Keyboard key pressed.
    /* Context usage:
     * EventContext::U16([key_code, ..]) => {
     * 
     * }
     */
    KeyPressed = 0x02,

    // Keyboard key released.
    /* Context usage:
     * EvenyContext::U16([key_code, ..]) => {
     * 
     * }
     */
    KeyReleased = 0x03,

    // Mouse button pressed.
    /* Context usage:
     * EventContext::U16([button, ..]) => {
     * 
     * }
     */
    ButtonPressed = 0x04,

    // Mouse button released.
    /* Context usage:
     * EventContext::U16([button, ..]) => {
     * 
     * }
     */
    ButtonReleased = 0x05,

    // Mouse moved.
    /* Context usage:
     * EventContext::U16([x, y, ..]) => {
     * 
     * }
     */
    MouseMoved = 0x06,

    // Mouse moved.
    /* Context usage:W
     * EventContext::U16([z_delta, ..]) => {
     * 
     * }
     */
    MouseWheel = 0x07,

    // Resized/resolution changed from the OS.
    /* Context usage:
     * EventContext::U16([width, height, ..]) => {
     * 
     * }
     */
    Resized = 0x08,
}
impl EventCode for SystemEventCode {
    fn code(&self) -> u32 {
        *self as u32
    }
}