use crate::utility;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use std::{ str, process::Command };

pub struct GenFrameController;

impl GenFrameController {
    pub fn active_submit(genframe: &GenFrame) {

    }
}

pub struct GenFrame {
    pub frame: gtk::Box,
    pub submit_btn: gtk::Button,
}

impl GenFrame {
    pub fn new() -> GenFrame {
        let frame = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let text_entry = gtk::Entry::new();
        let paths = utility::path_traversal();
        let combo = gtk::ComboBoxText::new();
        let submit_btn = gtk::Button::new_with_label("Submit");
        for path in paths {
            combo.append_text(&path);
        }
    
        text_entry.set_placeholder_text(Some("Enter public key file name here"));

        frame.pack_start(&combo, false, false, 0);
        frame.pack_start(&text_entry, false, false, 0);
        frame.pack_start(&submit_btn, false, false, 0);

        GenFrame { frame, submit_btn }
    }
}