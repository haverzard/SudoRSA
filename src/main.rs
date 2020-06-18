#[macro_use]
extern crate colour;
extern crate gio;
extern crate glib;
extern crate gtk;

mod utility;
mod mainframe;
mod genframe;

use atk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use std::env::args;

// fn create_main_window(application: &gtk::Application) -> gtk::ApplicationWindow {

// }

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let mainframe = mainframe::MainFrame::new();
    mainframe::MainFrameController::active_priv(&mainframe);
    mainframe::MainFrameController::active_gen_pub(&application, &mainframe);

    window.set_title("Sudo RSA");
    window.set_position(gtk::WindowPosition::Center);
    window.set_size_request(1000, 800);

    window.add(&mainframe.frame);
    window.show_all();
}

fn main() {
    utility::begin_check();

    let application =
        gtk::Application::new(Some("com.github.haverzard.rsapam"), Default::default())
            .expect("Initialization failed...");

    application.connect_activate(|app| {
        build_ui(app);
    });

    application.run(&args().collect::<Vec<_>>());
}