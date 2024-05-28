use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use once_cell::sync::Lazy;

use super::input::{Button, Key};

static INIT: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<Mutex<HashMap<&'static str, Vec<(Weak<dyn Send + Sync>, Box<On>)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));
static mut UNIT_LISTENER: Lazy<Listener> = Lazy::new(|| Listener::new(()));

type On =
    dyn Fn(Arc<dyn Sync + Send>, &dyn Event, Option<&(dyn Sync + Send)>) -> bool + Send + Sync;

pub(crate) fn init() -> Result<()> {
    if INIT.load(Ordering::Relaxed) {
        Err(Error)
    } else {
        INIT.store(true, Ordering::Relaxed);
        Ok(())
    }
}
pub(crate) fn close() {}

pub trait Event {
    fn get_name(&self) -> &'static str {
        std::any::type_name_of_val(self)
    }
    fn name() -> &'static str where Self: Sized {
        std::any::type_name::<Self>()
    }
}

pub trait Sender: Sync + Send {
    fn fire(&self, event: impl Event) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            for (l, c) in match STATE.lock().unwrap().get(event.get_name()) {
                Some(v) => v,
                None => {
                    return false;
                }
            } {
                if let Some(l) = l.upgrade() {
                    if c(l, &event, Some(&self)) {
                        return true;
                    }
                }
            }
            false
        }
    }
}
pub fn fire(event: impl Event) -> bool {
    if INIT.load(Ordering::Relaxed) {
        false
    } else {
        for (l, c) in match STATE.lock().unwrap().get(event.get_name()) {
            Some(v) => v,
            None => {
                return false;
            }
        } {
            if let Some(l) = l.upgrade() {
                if c(l, &event, None) {
                    return true;
                }
            }
        }
        false
    }
}

pub struct Listener {
    listener: Arc<dyn Send + Sync>,
    events: HashSet<&'static str>,
}
impl Listener {
    pub fn new<T: Send + Sync + 'static>(val: T) -> Self {
        Self {
            listener: Arc::new(val) as Arc<dyn Send + Sync>,
            events: HashSet::new(),
        }
    }
    pub fn register<E: Event>(&mut self, callback: Box<On>) -> bool {
        if INIT.load(Ordering::Relaxed) || self.events.contains(&E::name()) {
            false
        } else {
            match STATE.lock().unwrap().get_mut(E::name()) {
                Some(v) => v,
                None => {
                    return false;
                }
            }
            .push((
                Arc::<dyn Send + Sync>::downgrade(&self.listener.clone()),
                callback,
            ));
            self.events.insert(E::name());
            true
        }
    }
    pub fn unregister(&mut self, event: &'static str) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            match STATE.lock().unwrap().get_mut(event) {
                Some(v) => v,
                None => {
                    return false;
                }
            }
            .retain(|(w, _)| {
                if let Some(ref w) = w.upgrade() {
                    !Arc::ptr_eq(w, &self.listener)
                } else {
                    true
                }
            });
            self.events.remove(event);
            true
        }
    }
}
impl Drop for Listener {
    fn drop(&mut self) {
        let v = Vec::from_iter(self.events.drain());
        for e in v {
            self.unregister(e);
        }
    }
}

pub fn register<E: Event>(callback: Box<On>) -> bool {
    unsafe { UNIT_LISTENER.register::<E>(callback) }
}
pub fn unregister(event: &'static str) -> bool {
    unsafe { UNIT_LISTENER.unregister(event) }
}

pub struct ApplicationQuit;
impl Event for ApplicationQuit {}
pub struct KeyPress(pub Key);
impl Event for KeyPress {}
pub struct KeyRelease(pub Key);
impl Event for KeyRelease {}
pub struct ButtonPress(pub Button);
impl Event for ButtonPress {}
pub struct ButtonRelease(pub Button);
impl Event for ButtonRelease {}
pub struct MouseMove {
    pub x: i16,
    pub y: i16,
}
impl Event for MouseMove {}
pub struct MouseWheel(pub i8);
impl Event for MouseWheel {}
pub struct Resize {
    pub width: u16,
    pub height: u16,
}
impl Event for Resize {}

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error;
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Event Error)")
    }
}
impl std::error::Error for Error {}
