use crate::utility;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

pub struct GenFrame {
    pub frame: gtk::Box,
}

impl GenFrame {
    pub fn new() -> GenFrame {
        let frame = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let text_entry = gtk::Entry::new();

        text_entry.set_placeholder_text(Some("filename"));

        frame.pack_start(&text_entry, false, false, 0);

        GenFrame { frame }
    }
}