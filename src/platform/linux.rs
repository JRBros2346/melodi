// Linux platform layer.
use x11rb::connection::*;
use x11rb::protocol::xproto::*;
use x11rb::protocol::*;
use x11rb::rust_connection::*;

use std::rc::Rc;

pub(crate) struct PlatformState {
    connection: RustConnection,
    window: Window,
    screen: Rc<Screen>,
    protocols: Atom,
    delete_window: Atom,
}
impl PlatformState {
    pub(crate) fn startup(
        app_name: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, PlatformError> {
        // Retrieve the connection from the display.
        let (con, n) = match RustConnection::connect(None) {
            Ok(c) => c,
            Err(e) => {
                crate::fatal!("Failed to connect to X server via `x11rb`.");
                return Err(PlatformError::Connect(e));
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
        out.connection
            .change_keyboard_control(
                &ChangeKeyboardControlAux::new().auto_repeat_mode(Some(AutoRepeatMode::OFF)),
            )
            .map_err(PlatformError::Connection)?;
        // Get data from the X server
        let setup = out.connection.setup();

        // Assign screen
        out.screen = Rc::new(setup.roots[n].clone());

        // Allocate a XID for the window to be created.
        out.window = out
            .connection
            .generate_id()
            .map_err(PlatformError::RelyOrId)?;

        // Listen for keyboard and mouse buttons
        let event_values = EventMask::BUTTON_PRESS
            | EventMask::BUTTON_RELEASE
            | EventMask::KEY_PRESS
            | EventMask::KEY_RELEASE
            | EventMask::EXPOSURE
            | EventMask::POINTER_MOTION
            | EventMask::STRUCTURE_NOTIFY;

        // Create the window
        out.connection
            .create_window(
                x11rb::COPY_FROM_PARENT as u8,
                out.window,
                out.screen.root,
                x,
                y,
                width,
                height,
                0,
                WindowClass::INPUT_OUTPUT,
                out.screen.root_visual,
                &CreateWindowAux::new()
                    .background_pixel(out.screen.black_pixel)
                    .event_mask(Some(event_values)),
            )
            .map_err(PlatformError::Connection)?;

        // Change title
        use x11rb::wrapper::ConnectionExt;
        out.connection
            .change_property8(
                PropMode::REPLACE,
                out.window,
                AtomEnum::WM_NAME,
                AtomEnum::STRING,
                app_name.as_bytes(),
            )
            .map_err(PlatformError::Connection)?;

        // Tell the server to notify when the window manager
        // attempts to destroy the window.
        out.delete_window = out
            .connection
            .intern_atom(false, "WM_DELETE_WINDOW".as_bytes())
            .map_err(PlatformError::Connection)?
            .reply()
            .map_err(PlatformError::Reply)?
            .atom;
        out.protocols = out
            .connection
            .intern_atom(false, "WM_PROTOCOLS".as_bytes())
            .map_err(PlatformError::Connection)?
            .reply()
            .map_err(PlatformError::Reply)?
            .atom;

        out.connection
            .change_property32(
                PropMode::REPLACE,
                out.window,
                out.protocols,
                AtomEnum::ATOM,
                &[out.delete_window],
            )
            .map_err(PlatformError::Connection)?;

        // Map the window to the screen
        out.connection
            .map_window(out.window)
            .map_err(PlatformError::Connection)?;

        // Flush the stream
        if let Err(e) = out.connection.flush() {
            crate::fatal!("An error occurred when flusing the stream");
            return Err(PlatformError::Connection(e));
        }

        Ok(out)
    }
    pub(crate) fn shutdown(&mut self) -> Result<(), PlatformError> {
        // Turn key repeats back on (It's global for the OS)
        self.connection
            .change_keyboard_control(
                &ChangeKeyboardControlAux::new().auto_repeat_mode(Some(AutoRepeatMode::ON)),
            )
            .map_err(PlatformError::Connection)?;

        self.connection
            .destroy_window(self.window)
            .map_err(PlatformError::Connection)?;

        Ok(())
    }
    pub(crate) fn pump_messages(&self) -> Result<bool, PlatformError> {
        let mut quit = false;
        // Poll for events until None is returned.
        loop {
            match self
                .connection
                .poll_for_event()
                .map_err(PlatformError::Connection)?
            {
                None => break,
                Some(e) => match e {
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
            };
        }
        Ok(!quit)
    }
}

pub enum PlatformError {
    Connect(ConnectError),
    Connection(ConnectionError),
    RelyOrId(ReplyOrIdError),
    Reply(ReplyError),
}
impl std::fmt::Debug for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => write!(f, "{:?}", e),
            Self::Connection(e) => write!(f, "{:?}", e),
            Self::RelyOrId(e) => write!(f, "{:?}", e),
            Self::Reply(e) => write!(f, "{:?}", e),
        }
    }
}
impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connect(e) => write!(f, "{}", e),
            Self::Connection(e) => write!(f, "{}", e),
            Self::RelyOrId(e) => write!(f, "{}", e),
            Self::Reply(e) => write!(f, "{}", e),
        }
    }
}
impl std::error::Error for PlatformError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Connect(e) => Some(e),
            Self::Connection(e) => Some(e),
            Self::RelyOrId(e) => Some(e),
            Self::Reply(e) => Some(e),
        }
    }
}
