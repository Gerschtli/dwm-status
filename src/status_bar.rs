#![allow(unsafe_code)]

use async;
use error::*;
use feature;
use settings;
use std::collections::HashMap;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;
use uuid;
use x11::xlib;

pub struct StatusBar {
    feature_map: HashMap<uuid::Uuid, Box<dyn feature::Feature>>,
    order: Vec<uuid::Uuid>,
    string_map: HashMap<uuid::Uuid, String>,
    xsetroot: XSetRoot,
}

impl StatusBar {
    pub fn new(features: Vec<Box<dyn feature::Feature>>) -> Result<Self> {
        let order: Vec<_> = features.iter().map(|feature| feature.id()).collect();

        let string_map: HashMap<_, _> = features
            .iter()
            .map(|feature| (feature.id(), String::new()))
            .collect();

        let feature_map: HashMap<_, _> = features
            .into_iter()
            .map(|feature| (feature.id(), feature))
            .collect();

        Ok(StatusBar {
            feature_map,
            order,
            string_map,
            xsetroot: XSetRoot::new()?,
        })
    }

    pub fn update(
        &mut self,
        message: &async::Message,
        settings: &settings::Settings,
    ) -> Result<()> {
        match message {
            async::Message::FeatureUpdate(id) if self.feature_map.contains_key(id) => {
                self.update_feature(*id, &settings)?;
                self.render(&settings)?;
            },
            async::Message::FeatureUpdate(id) => {
                return Err(Error::new_custom(
                    "invalid message",
                    &format!("message id {} does not exist", id),
                ));
            },
            async::Message::UpdateAll => {
                if settings.debug {
                    println!("update all");
                }

                for id in self.order.clone() {
                    self.update_feature(id, &settings)?;
                }
                self.render(&settings)?;
            },
            _ => (),
        }

        Ok(())
    }

    fn update_feature(&mut self, id: uuid::Uuid, settings: &settings::Settings) -> Result<()> {
        let feature = self.feature_map.get_mut(&id).unwrap();
        let rendered = feature.update()?.render(&settings);

        if settings.debug {
            println!("update {}: {}", feature.name(), &rendered);
        }

        self.string_map.insert(id, rendered);
        Ok(())
    }

    pub fn render(&self, settings: &settings::Settings) -> Result<()> {
        let status = self
            .order
            .iter()
            .map(|id| &self.string_map[id][..])
            .collect::<Vec<_>>()
            .join(&settings.separator);

        self.xsetroot.render(status)
    }
}

struct XSetRoot {
    display: *mut xlib::Display,
    root_window: xlib::Window,
}

impl XSetRoot {
    fn new() -> Result<Self> {
        unsafe {
            let display = xlib::XOpenDisplay(ptr::null());

            if display.is_null() {
                return Err(Error::new_custom("render", "cannot open display"));
            }

            let screen = xlib::XDefaultScreen(display);
            let root_window = xlib::XRootWindow(display, screen);

            Ok(XSetRoot {
                display,
                root_window,
            })
        }
    }

    fn render(&self, text: String) -> Result<()> {
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
