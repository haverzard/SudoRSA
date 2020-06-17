#[macro_use]
extern crate colour;
extern crate gio;
extern crate glib;
extern crate gtk;

mod utility;

use atk::prelude::*;
use gio::prelude::*;
use glib::clone;
use gtk::prelude::*;

use std::env::args;

// fn create_main_window(application: &gtk::Application) -> gtk::ApplicationWindow {

// }

fn gen_priv_key(application: &gtk::Application) {
    utility::gen_priv_key();
    let sub_window = gtk::Window::new(gtk::WindowType::Toplevel);
    application.add_window(&sub_window);
    sub_window.set_title("Private Key Generation");
    sub_window.set_position(gtk::WindowPosition::Center);
    sub_window.set_default_size(200, 100);
    let res = gtk::Label::new(Some("Completed!"));
    sub_window.add(&res);
    sub_window.show_all();
}

fn gen_pub_key(application: &gtk::Application) {
    //utility::gen_priv_key();
    let sub_window = gtk::Window::new(gtk::WindowType::Toplevel);
    application.add_window(&sub_window);
    sub_window.set_title("Pub Key Generation");
    sub_window.set_position(gtk::WindowPosition::Center);
    sub_window.set_default_size(400, 200);
    let res = gtk::Label::new(Some("Completed!"));
    let entry = gtk::Entry::new();
    sub_window.add(&entry);
    sub_window.show_all();
}

fn build_ui(application: &gtk::Application) {
    let window = gtk::ApplicationWindow::new(application);

    window.set_title("Sudo RSA");
    window.set_position(gtk::WindowPosition::Center);
    window.set_size_request(1000, 800);

    let outer_box = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let title_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
    let title = gtk::Label::new(Some("Sudo RSA"));
    let gen_priv_btn = gtk::Button::new_with_label("Generate private key");
    let gen_pub_btn = gtk::Button::new_with_label("Generate public key");
    let check_pub_btn = gtk::Button::new_with_label("Check public key");

    title_box.pack_start(&title, false, false, 0);
    title_box.set_size_request(200, 200);
    v_box.set_size_request(500, 800);
    gen_priv_btn.set_size_request(200, 50);

    gen_priv_btn.connect_clicked(clone!(@weak application => move |_| { gen_priv_key(&application); }));
    gen_pub_btn.connect_clicked(clone!(@weak application => move |_| { gen_pub_key(&application); }));

    v_box.pack_start(&title_box, false, false, 0);
    v_box.pack_start(&gen_priv_btn, false, false, 0);
    v_box.pack_start(&gen_pub_btn, false, false, 0);
    v_box.pack_start(&check_pub_btn, false, false, 0);

    let dummy_left = gtk::Box::new(gtk::Orientation::Horizontal, 10);
    dummy_left.set_size_request(250,800);
    outer_box.pack_start(&dummy_left, false, false, 0);
    outer_box.pack_start(&v_box, false, false, 0);
    window.add(&outer_box);
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