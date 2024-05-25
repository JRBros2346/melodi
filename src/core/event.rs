use std::collections::{HashMap, HashSet};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use once_cell::sync::Lazy;

static INIT: AtomicBool = AtomicBool::new(false);
static STATE: Lazy<Mutex<HashMap<&'static str, Vec<(Weak<dyn Send + Sync>, Box<On>)>>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

type On = dyn Fn(Arc<dyn Sync + Send>, &dyn Event) -> bool + Send + Sync;

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
}

pub trait Sender {
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
                    if c(l, &event) {
                        return true;
                    }
                }
            }
            false
        }
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
    pub fn register(&mut self, event: &'static str, callback: Box<On>) -> bool {
        if INIT.load(Ordering::Relaxed) || self.events.contains(&event) {
            false
        } else {
            match STATE.lock().unwrap().get_mut(event) {
                Some(v) => v,
                None => {
                    return false;
                }
            }
            .push((
                Arc::<dyn Send + Sync>::downgrade(&self.listener.clone()),
                callback,
            ));
            self.events.insert(event);
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

pub type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
pub struct Error;
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(Event Error)")
    }
}
impl std::error::Error for Error {}