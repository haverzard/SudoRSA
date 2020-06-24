use crate::{ utility, device };
use std::{ fs };
use glib::clone;
use gtk::prelude::*;

pub struct GenFrameController;

impl GenFrameController {
    pub fn active_submit(window: &gtk::Window, genframe: &GenFrame, res_label: &gtk::Label) {
        let device_entry = &genframe.device_entry;
        let filename_entry = &genframe.filename_entry;
        genframe.submit_btn.connect_clicked(clone!(@weak window, @weak device_entry, @weak filename_entry, @weak res_label => move |_| {
            match device_entry.get_active_text() {
                Some(device_path) => {
                    match filename_entry.get_text() {
                        Some(file_path) => {
                            let trimmed_file_path = file_path.trim();
                            if trimmed_file_path != "" {
                                window.close();
                                res_label.set_text("Generating new public key...");
                                let _ = fs::create_dir(format!("{}/keys", device_path));
                                utility::pub_key_generator(format!("{}/keys/{}", device_path, file_path.trim()));
                                res_label.set_text("Public key generation completes.");
                            }
                        },
                        None => ()
                    }
                },
                None => ()
            };
        }));
    }
}

pub struct GenFrame {
    pub frame: gtk::Box,
    pub device_entry: gtk::ComboBoxText,
    pub filename_entry: gtk::Entry,
    pub submit_btn: gtk::Button,
}

impl GenFrame {
    pub fn new() -> GenFrame {
        let frame = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let paths = device::path_traversal();
        let device_entry = gtk::ComboBoxText::new();
        let filename_entry = gtk::Entry::new();
        let submit_btn = gtk::Button::new_with_label("Submit");
        for path in paths {
            device_entry.append_text(&path);
        }
    
        filename_entry.set_placeholder_text(Some("Enter public key file name here"));

        frame.pack_start(&device_entry, false, false, 0);
        frame.pack_start(&filename_entry, false, false, 0);
        frame.pack_start(&submit_btn, false, false, 0);

        GenFrame { frame, device_entry, filename_entry, submit_btn }
    }
}