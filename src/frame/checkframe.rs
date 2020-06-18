use crate::{ utility, device };
use glib::clone;
use gtk::prelude::*;

pub struct CheckFrameController;

impl CheckFrameController {
    pub fn active_submit(window: &gtk::Window, genframe: &CheckFrame, res_label: &gtk::Label) {
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
                                res_label.set_text("Checking public key...");
                                match utility::pub_key_checker(format!("{}/keys/{}", device_path, file_path.trim())) {
                                    Some(_) => res_label.set_text("Public key is correct..."),
                                    None => res_label.set_text("Public key is not correct...")
                                }
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

pub struct CheckFrame {
    pub frame: gtk::Box,
    pub device_entry: gtk::ComboBoxText,
    pub filename_entry: gtk::Entry,
    pub submit_btn: gtk::Button,
}

impl CheckFrame {
    pub fn new() -> CheckFrame {
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

        CheckFrame { frame, device_entry, filename_entry, submit_btn }
    }
}