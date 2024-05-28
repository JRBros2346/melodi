// Windows platform layer.
use windows::core::*;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

use crate::core::input::{self, Button, Key};

pub(crate) struct PlatformState {
    instance: HINSTANCE,
    window: HWND,
}
impl PlatformState {
    pub(crate) fn startup(app_name: &str, x: i16, y: i16, width: u16, height: u16) -> Result<Self> {
        unsafe {
            let mut out = Self {
                instance: HINSTANCE::default(),
                window: HWND::default(),
            };
            GetModuleHandleExW(0, None, &mut out.instance.into() as *mut _).map_err(Error)?;

            // Setup and register window class.
            if RegisterClassExW(&WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_DBLCLKS, // Get double-clicks
                lpfnWndProc: Some(Self::win32_process_messages),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: out.instance,
                hIcon: LoadIconW(out.instance, IDI_APPLICATION).map_err(Error)?,
                hCursor: LoadCursorW(None, IDC_ARROW).map_err(Error)?,
                hbrBackground: HBRUSH(0), // Transparent
                lpszMenuName: PCWSTR::null(),
                lpszClassName: w!("strings_window_class"),
                hIconSm: HICON(0),
            } as *const _)
                == 0
            {
                MessageBoxW(
                    None,
                    w!("Window registration failed"),
                    w!("Error"),
                    MB_ICONEXCLAMATION | MB_OK,
                );
                return Err(Error(windows::core::Error::from_win32()));
            }

            // Create window
            let client_x = x as i32;
            let client_y = y as i32;
            let client_width = width as i32;
            let client_height = height as i32;

            let mut window_x = client_x;
            let mut window_y = client_y;
            let mut window_width = client_width;
            let mut window_height = client_height;

            let mut window_style = WS_OVERLAPPED | WS_SYSMENU | WS_CAPTION;
            let window_ex_style = WS_EX_APPWINDOW;

            window_style |= WS_MAXIMIZEBOX;
            window_style |= WS_MINIMIZEBOX;
            window_style |= WS_THICKFRAME;

            // Obtain the size of the border.
            let mut border = RECT::default();
            AdjustWindowRectEx(&mut border as *mut _, window_style, None, window_ex_style)
                .map_err(Error)?;

            // In this case, the border rectangle is negative.
            window_x += border.left;
            window_y += border.top;

            // Grow by the size of the OS border.
            window_width += border.right - border.left;
            window_height += border.bottom - border.top;

            out.window = CreateWindowExW(
                window_ex_style,
                w!("strings_window_class"),
                PCWSTR::from_raw(
                    app_name
                        .encode_utf16()
                        .chain(std::iter::repeat(0).take(1))
                        .collect::<Vec<_>>()
                        .as_ptr(),
                ),
                window_style,
                window_x,
                window_y,
                window_width,
                window_height,
                None,
                None,
                out.instance,
                None,
            );
            if out.window.0 == 0 {
                MessageBoxW(
                    None,
                    w!("Window creation failed!"),
                    w!("Error!"),
                    MB_ICONEXCLAMATION | MB_OK,
                );
                crate::fatal!("Window creation failed!");
                return Err(Error(windows::core::Error::from_win32()));
            }

            // Show the window
            let should_activate = true; // TODO: if the window should not accept input, this should be `false`.
            ShowWindow(
                out.window,
                // If initially minimized, use `{SW_MINIMIZE} else {SW_SHOWMINNOACTIVE}`
                // If initially maximized, use `{SW_SHOWMAXIMIZED} else {SW_MAXIMIZE}`
                if should_activate {
                    SW_SHOW
                } else {
                    SW_SHOWNOACTIVATE
                },
            );
            let _ = colored::control::set_virtual_terminal(true);

            Ok(out)
        }
    }
    pub(crate) fn shutdown(&mut self) -> Result<()> {
        unsafe {
            if self.window.0 != 0 {
                DestroyWindow(self.window).map_err(Error)?;
                self.window = HWND::default();
            }
            Ok(())
        }
    }
    pub(crate) fn pump_messages(&self) -> Result<bool> {
        unsafe {
            let mut message = MSG::default();
            while PeekMessageW(&mut message as *mut _, None, 0, 0, PM_REMOVE).0 > 0 {
                TranslateMessage(&message as *const _);
                DispatchMessageW(&message as *const _);
            }
            Ok(true)
        }
    }
    unsafe extern "system" fn win32_process_messages(
        window: HWND,
        message: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        match message {
            WM_ERASEBKGND => {
                // Notify the OS that erasing will be handled by the application to prevent flicker.
                return LRESULT(1);
            }
            WM_CLOSE => {
                // TODO: Fire an event for the application to quit.
                return LRESULT(0);
            }
            WM_DESTROY => {
                PostQuitMessage(0);
                return LRESULT(0);
            }
            WM_SIZE => {
                // Get the updated size.
                // let mut r = RECT::default();
                // unsafe {
                //     GetClientRect(window, &mut r as *mut _).unwrap();
                // }
                // let width = (r.right - r.left) as u32;
                // let height = (r.bottom - r.top) as u32;
                // TODO: Fire an event for window resize.
            }
            WM_KEYDOWN | WM_SYSKEYDOWN | WM_KEYUP | WM_SYSKEYUP => {
                // Key pressed/released
                let press = message == WM_KEYDOWN || message == WM_SYSKEYDOWN;
                if let Some(key) = Key::from_repr(wparam.0 as u8) {
                    // Pass to input subsystem.
                    input::process_key(key, press);
                }
            }
            WM_MOUSEMOVE => {
                // Mouse move
                let x = (lparam.0 & 0xffff) as i16;
                let y = ((lparam.0 >> 16) & 0xffff) as i16;

                // Pass the value to subsystem
                input::process_mouse_move(x, y);
            }
            WM_MOUSEWHEEL => {
                let mut z_delta = ((wparam.0 >> 16) & 0xffff) as i8;
                if z_delta != 0 {
                    // Flatten the input to an OS-independent (-1, 1)
                    z_delta = if z_delta < 0 {-1} else {1};
                    input::process_mouse_wheel(z_delta);
                }
            }
            WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONUP
            | WM_RBUTTONUP => {
                let press = message == WM_LBUTTONDOWN || message == WM_RBUTTONDOWN || message == WM_MBUTTONDOWN;
                let mut button: Option<Button> = None;
                match message {
                    WM_LBUTTONDOWN | WM_LBUTTONUP => {
                        button = Some(Button::Left);
                    }
                    WM_MBUTTONDOWN | WM_MBUTTONUP => {
                        button = Some(Button::Middle);
                    }
                    WM_RBUTTONDOWN | WM_RBUTTONUP => {
                        button = Some(Button::Right);
                    }
                    _ => {}
                }
                
                // Pass over to the input subsystem.
                if let Some(button) = button {
                    input::process_button(button, press);
                }
            }
            _ => {}
        }
        DefWindowProcW(window, message, wparam, lparam)
    }
}

pub type Result<T> = std::result::Result<T, Error>;
pub struct Error(windows::core::Error);
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
