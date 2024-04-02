// Linux platform layer.
use x11rb::connection::*;
use x11rb::protocol::xproto::*;
use x11rb::protocol::*;
use x11rb::rust_connection::*;

use std::rc::Rc;
use std::result::Result;

pub struct PlatformState {
    connection: RustConnection,
    window: Window,
    screen: Rc<Screen>,
    protocols: Atom,
    delete_window: Atom,
}
impl PlatformState {
    pub fn startup(
        app_name: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<Self, String> {
        // Retrieve the connection from the display.
        let (con, n) = match RustConnection::connect(None) {
            Ok(c) => c,
            Err(e) => {
                crate::fatal!("Failed to connect to X server via XCB.");
                return Err(e.to_string());
            }
        };
        let mut out = Self {
            connection: con,
            window: Window::default(),
            screen: Rc::new(Screen::default()),
            protocols: Atom::default(),
            delete_window: Atom::default(),
        };

        // Turn off key repeats.
        if let Err(e) = out.connection.change_keyboard_control(
            &ChangeKeyboardControlAux::new().auto_repeat_mode(Some(AutoRepeatMode::OFF)),
        ) {
            return Err(e.to_string());
        }

        // Get data from the X server
        let setup = out.connection.setup();

        // Assign screen
        out.screen = Rc::new(setup.roots[n].clone());

        // Allocate a XID for the window to be created.
        out.window = match out.connection.generate_id() {
            Ok(w) => w,
            Err(e) => {
                return Err(e.to_string());
            }
        };

        // Listen for keyboard and mouse buttons
        let event_values = EventMask::BUTTON_PRESS
            | EventMask::BUTTON_RELEASE
            | EventMask::KEY_PRESS
            | EventMask::KEY_RELEASE
            | EventMask::EXPOSURE
            | EventMask::POINTER_MOTION
            | EventMask::STRUCTURE_NOTIFY;

        // Create the window
        if let Err(e) = out.connection.create_window(
            x11rb::COPY_FROM_PARENT as u8,
            out.window,
            out.screen.root,
            x as i16,
            y as i16,
            width as u16,
            height as u16,
            0,
            WindowClass::INPUT_OUTPUT,
            out.screen.root_visual,
            &CreateWindowAux::new()
                .background_pixel(out.screen.black_pixel)
                .event_mask(Some(event_values)),
        ) {
            return Err(e.to_string());
        }

        // Change title
        use x11rb::wrapper::ConnectionExt;
        if let Err(e) = out.connection.change_property8(
            PropMode::REPLACE,
            out.window,
            AtomEnum::WM_NAME,
            AtomEnum::STRING,
            app_name.as_bytes(),
        ) {
            return Err(e.to_string());
        }

        // Tell the server to notify when the window manager
        // attempts to destroy the window.
        out.delete_window = match out
            .connection
            .intern_atom(false, "WM_DELETE_WINDOW".as_bytes())
        {
            Ok(atom) => match atom.reply() {
                Ok(reply) => reply.atom,
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            Err(e) => {
                return Err(e.to_string());
            }
        };
        out.protocols = match out.connection.intern_atom(false, "WM_PROTOCOLS".as_bytes()) {
            Ok(atom) => match atom.reply() {
                Ok(reply) => reply.atom,
                Err(e) => {
                    return Err(e.to_string());
                }
            },
            Err(e) => {
                return Err(e.to_string());
            }
        };

        if let Err(e) = out.connection.change_property32(
            PropMode::REPLACE,
            out.window,
            out.protocols,
            AtomEnum::ATOM,
            &[out.delete_window],
        ) {
            return Err(e.to_string());
        }

        // Map the window to the screen
        if let Err(e) = out.connection.map_window(out.window) {
            return Err(e.to_string());
        }

        // Flush the stream
        if let Err(e) = out.connection.flush() {
            crate::fatal!(
                "An error occurred when flusing the stream: {}",
                e.to_string()
            );
        }

        Ok(out)
    }
    pub fn shutdown(&mut self) -> Result<(), String> {
        // Turn key repeats back on (It's global for the OS)
        if let Err(e) = self.connection.change_keyboard_control(
            &ChangeKeyboardControlAux::new().auto_repeat_mode(Some(AutoRepeatMode::ON)),
        ) {
            return Err(e.to_string());
        }

        if let Err(e) = self.connection.destroy_window(self.window) {
            return Err(e.to_string());
        }

        Ok(())
    }
    pub fn pump_messages(&self) -> Result<bool, String> {
        let mut quit = false;
        // Poll for events until None is returned.
        loop {
            match self.connection.poll_for_event() {
                Ok(None) => break,
                Ok(Some(e)) => match e {
                    Event::KeyPress(_) | Event::KeyRelease(_) => {
                        // TODO: Key presses and releases
                    }
                    Event::ButtonPress(_) | Event::ButtonRelease(_) => {
                        // TODO: Mouse button presses and releases
                    }
                    Event::MotionNotify(_) => {
                        // TODO: Mouse movement
                    }
                    Event::ConfigureNotify(_) => {
                        // TODO: Resizing
                    }
                    Event::ClientMessage(cm) => {
                        if cm.data.as_data32()[0] == self.delete_window {
                            quit = true;
                        }
                    }
                    _ => {
                        // Something else
                    }
                },
                Err(e) => {
                    return Err(e.to_string());
                }
            };
        }
        Ok(!quit)
    }
}
