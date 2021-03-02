#![allow(unsafe_code)]

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use x11::xlib;

use crate::error::Error;
use crate::error::Result;
use crate::error::WrapErrorExt;

pub(crate) struct XSetRoot {
    display: *mut xlib::Display,
    root_window: xlib::Window,
}

impl XSetRoot {
    pub(crate) fn init() -> Result<Self> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());

            if display.is_null() {
                return Err(Error::new_custom("render", "cannot open display"));
            }

            let screen = xlib::XDefaultScreen(display);
            let root_window = xlib::XRootWindow(display, screen);

            Ok(Self {
                display,
                root_window,
            })
        }
    }

    pub(crate) fn render(&self, text: String) -> Result<()> {
        let status_c = CString::new(text)
            .wrap_error("render", "status text could not be converted to CString")?;

        unsafe {
            xlib::XStoreName(
                self.display,
                self.root_window,
                status_c.as_ptr() as *mut c_char,
            );

            xlib::XFlush(self.display);
        }

        Ok(())
    }
}

impl Drop for XSetRoot {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
