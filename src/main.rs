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

fn gen_pub_key(application: &gtk::Application) {
    //utility::gen_priv_key();
    let sub_window = gtk::Window::new(gtk::WindowType::Toplevel);
    let genframe = genframe::GenFrame::new();

    sub_window.set_title("Pub Key Generation");
    sub_window.set_position(gtk::WindowPosition::Center);
    sub_window.set_default_size(400, 200);
    
    
    sub_window.add(&genframe.frame);
    
    application.add_window(&sub_window);
    
    sub_window.show_all();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    let mainframe = mainframe::MainFrame::new();
    mainframe::MainFrameController::active_priv(&mainframe);

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