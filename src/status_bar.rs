#![allow(unsafe_code)]

use error::*;
use feature;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use x11::xlib;

pub struct StatusBar {
    display: *mut xlib::Display,
    root_window: xlib::Window,
    separator: String,
}

impl StatusBar {
    pub fn new(separator: String) -> Result<Self> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());

            if display.is_null() {
                return Err(Error::new_custom("render", "cannot open display"));
            }

            let screen = xlib::XDefaultScreen(display);
            let root_window = xlib::XRootWindow(display, screen);

            Ok(StatusBar {
                display,
                root_window,
                separator,
            })
        }
    }

    pub fn render(
        &self,
        order: &[String],
        feature_map: &HashMap<String, Box<feature::Feature>>,
    ) -> Result<()> {
        let status = order
            .iter()
            .map(|id| feature_map.get(id).unwrap().render())
            .collect::<Vec<_>>()
            .join(&self.separator);

        let status_c = CString::new(status)
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

impl Drop for StatusBar {
    fn drop(&mut self) {
        unsafe {
            xlib::XCloseDisplay(self.display);
        }
    }
}
