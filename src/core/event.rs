use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex, Weak};

use once_cell::sync::Lazy;

static INIT: AtomicBool = AtomicBool::new(false);
static STATE: Mutex<Lazy<HashMap<&'static str, Vec<(Weak<dyn Send + Sync>, Box<On>)>>>> = Mutex::new(Lazy::new(|| HashMap::new()));

pub trait Sender {
    fn fire(&self, event: impl Event) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            for (l, c) in match STATE.lock().unwrap().get(event.get_name()) {
                Some(v) => v,
                None => { return false; }
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

pub trait Event {
    fn get_name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
}
pub struct Listener {
    listener: Arc<dyn Send + Sync>,
    events: Vec<&'static str>
}
impl Listener {
    pub fn new<T: Send + Sync + 'static>(val: T) -> Self {
        Self { 
            listener: Arc::new(val) as Arc<dyn Send + Sync>,
            events: Vec::new(),
        }
    }
    pub fn register(&mut self, event: impl Event, callback: Box<On>) -> bool {
        if INIT.load(Ordering::Relaxed) || self.events.contains(&event.get_name()) {
            false
        } else {
            self.events.push(event.get_name());
            match STATE.lock().unwrap().get_mut(event.get_name()) {
                Some(v) => v,
                None => { return false; }
            }.push((Arc::<dyn Send + Sync>::downgrade(&self.listener.clone()), callback));
            true
        }
    }
    pub fn unregister(&mut self, event: &'static str) -> bool {
        if INIT.load(Ordering::Relaxed) {
            false
        } else {
            let i = match match STATE.lock().unwrap().get_mut(event) {
                Some(v) => v,
                None => { return false; }
            }.iter().position(|(a, _)| Arc::ptr_eq(&self.listener, &match a.upgrade() {
                Some(arc) => arc,
                None => { return false; }
            })) {
                Some(i) => i,
                None => { return false; }
            };
            let _ = match STATE.lock().unwrap().get_mut(event) {
                Some(v) => v,
                None => { return false; }
            }.remove(i);
            let i = match self.events.iter().position(|&a| a==event) {
                Some(i) => i,
                None => { return false; }
            };
            self.events.remove(i);
            true
        }
    }
}

type On = dyn Fn(Arc<dyn Sync + Send>, &dyn Event) -> bool + Send + Sync;

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