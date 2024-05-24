// Windows platform layer.
use std::result::Result;
use windows::core;
use windows::Win32::Foundation::*;
use windows::Win32::Graphics::Gdi::*;
use windows::Win32::System::LibraryLoader::*;
use windows::Win32::UI::WindowsAndMessaging::*;

pub(crate) struct PlatformState {
    instance: HINSTANCE,
    window: HWND,
}
impl PlatformState {
    pub(crate) fn startup(
        app_name: &str,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
    ) -> Result<Self, PlatformError> {
        unsafe {
            let mut out = Self {
                instance: HINSTANCE::default(),
                window: HWND::default(),
            };
            GetModuleHandleExW(0, None, &mut out.instance.into() as *mut _)
                .map_err(PlatformError)?;

            // Setup and register window class.
            if RegisterClassExW(&WNDCLASSEXW {
                cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
                style: CS_DBLCLKS, // Get double-clicks
                lpfnWndProc: Some(Self::win32_process_messages),
                cbClsExtra: 0,
                cbWndExtra: 0,
                hInstance: out.instance,
                hIcon: LoadIconW(out.instance, IDI_APPLICATION).map_err(PlatformError)?,
                hCursor: LoadCursorW(None, IDC_ARROW).map_err(PlatformError)?,
                hbrBackground: HBRUSH(0), // Transparent
                lpszMenuName: core::PCWSTR::null(),
                lpszClassName: core::w!("strings_window_class"),
                hIconSm: HICON(0),
            } as *const _)
                == 0
            {
                MessageBoxW(
                    None,
                    core::w!("Window registration failed"),
                    core::w!("Error"),
                    MB_ICONEXCLAMATION | MB_OK,
                );
                return Err(PlatformError(core::Error::from_win32()));
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
                .map_err(PlatformError)?;

            // In this case, the border rectangle is negative.
            window_x = window_x + border.left;
            window_y = window_y + border.top;

            // Grow by the size of the OS border.
            window_width = window_width + (border.right - border.left);
            window_height = window_height + (border.bottom - border.top);

            out.window = CreateWindowExW(
                window_ex_style,
                core::w!("strings_window_class"),
                core::PCWSTR::from_raw(
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
                    core::w!("Window creation failed!"),
                    core::w!("Error!"),
                    MB_ICONEXCLAMATION | MB_OK,
                );
                crate::fatal!("Window creation failed!");
                return Err(PlatformError(core::Error::from_win32()));
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
    pub(crate) fn shutdown(&mut self) -> Result<(), PlatformError> {
        unsafe {
            if self.window.0 != 0 {
                DestroyWindow(self.window).map_err(PlatformError)?;
                self.window = HWND::default();
            }
            Ok(())
        }
    }
    pub(crate) fn pump_messages(&self) -> Result<bool, PlatformError> {
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
                // let pressed = message == WM_KEYDOWN || message == WM_SYSKEYDOWN;
                // TODO: input processing
            }
            WM_MOUSEMOVE => {
                // Mouse move
                // let x_position = (lparam.0 & 0xffff) as i32;
                // let y_position = ((lparam.0 >> 16) & 0xffff) as i32;
                // TODO: input processing.
            }
            WM_MOUSEWHEEL => {
                // let z_delta = ((wparam.0 >> 16) & 0xffff) as i32;
                // if z_delta != 0 {
                //     // Flatten the input to an OS-independent (-1, 1)
                //     z_delta = if z_delta < 0 {-1} else {1};
                //     // TODO: input processing.
                // }
            }
            WM_LBUTTONDOWN | WM_MBUTTONDOWN | WM_RBUTTONDOWN | WM_LBUTTONUP | WM_MBUTTONUP
            | WM_RBUTTONUP => {
                // let pressed = message == WM_LBUTTONDOWN || message == WM_RBUTTONDOWN || message == WM_MBUTTONDOWN;
                // TODO: input processing.
            }
            _ => {}
        }
        DefWindowProcW(window, message, wparam, lparam)
    }
}

pub struct PlatformError(pub(self) windows::core::Error);
impl std::fmt::Debug for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.0)
    }
}
impl std::fmt::Display for PlatformError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl std::error::Error for PlatformError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}
