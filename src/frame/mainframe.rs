use crate::{ utility, frame::genframe, frame::checkframe };
use glib::clone;
use gtk::prelude::*;

pub struct MainFrameController;

impl MainFrameController {
    pub fn active_priv(mainframe: &MainFrame) {
        let res_label = &mainframe.res_label;
        mainframe.gen_priv_btn.connect_clicked(clone!(@weak res_label => move |_| {
            res_label.set_text("Generating new private key...");
            utility::gen_priv_key();
            res_label.set_text("Private key generation completes.");
        }));
    }

    pub fn active_gen_pub(application: &gtk::Application, mainframe: &MainFrame) {
        let res_label = &mainframe.res_label;
        mainframe.gen_pub_btn.connect_clicked(clone!(@weak application, @weak res_label => move |_| {
            let sub_window = gtk::Window::new(gtk::WindowType::Toplevel);
            let genframe = genframe::GenFrame::new();
            genframe::GenFrameController::active_submit(&sub_window, &genframe, &res_label);

            sub_window.set_title("Public Key Generator");
            sub_window.set_position(gtk::WindowPosition::Center);
            sub_window.set_default_size(400, 100);

            sub_window.add(&genframe.frame);

            application.add_window(&sub_window);

            sub_window.show_all();
        }));
    }

    pub fn active_check_pub(application: &gtk::Application, mainframe: &MainFrame) {
        let res_label = &mainframe.res_label;
        mainframe.check_pub_btn.connect_clicked(clone!(@weak application, @weak res_label => move |_| {
            let sub_window = gtk::Window::new(gtk::WindowType::Toplevel);
            let checkframe = checkframe::CheckFrame::new();
            checkframe::CheckFrameController::active_submit(&sub_window, &checkframe, &res_label);

            sub_window.set_title("Public Key Checker");
            sub_window.set_position(gtk::WindowPosition::Center);
            sub_window.set_default_size(400, 100);

            sub_window.add(&checkframe.frame);

            application.add_window(&sub_window);

            sub_window.show_all();
        }));
    }
}

pub struct MainFrame {
    pub frame: gtk::Box,
    gen_priv_btn: gtk::Button,
    gen_pub_btn: gtk::Button,
    check_pub_btn: gtk::Button,
    res_label: gtk::Label,
}

impl MainFrame {
    pub fn new() -> MainFrame {
        let frame = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        let v_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let title_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let btn_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let res_box = gtk::Box::new(gtk::Orientation::Vertical, 10);
        let res_label = gtk::Label::new(None);
        let title = gtk::Label::new(None);
        let gen_priv_btn = MainFrame::gen_button("Generate private key");
        let gen_pub_btn = MainFrame::gen_button("Generate public key");
        let check_pub_btn = MainFrame::gen_button("Check public key");

        // gen_pub_btn.connect_clicked(clone!(@weak application => move |_| { gen_pub_key(&application); }));

        res_box.set_size_request(500, 300);
        res_box.pack_end(&res_label, false, false, 0);
        
        title.set_markup("<span font_desc=\"50\">Sudo RSA</span>");
        title_box.pack_end(&title, false, false, 0);
        title_box.set_size_request(500, 200);

        btn_box.pack_end(&check_pub_btn, false, false, 0);
        btn_box.pack_end(&gen_pub_btn, false, false, 0);
        btn_box.pack_end(&gen_priv_btn, false, false, 0);
        btn_box.set_size_request(500, 300);

        v_box.set_size_request(500, 800);
        v_box.set_margin_start(250);
        v_box.set_margin_end(250);
        v_box.pack_start(&title_box, false, false, 0);
        v_box.pack_start(&btn_box, false, false, 0);
        v_box.pack_start(&res_box, false, false, 0);

        frame.pack_start(&v_box, false, false, 0);

        MainFrame { frame, gen_priv_btn, gen_pub_btn, check_pub_btn, res_label }
    }
    fn gen_button(label: &str) -> gtk::Button {
        let btn = gtk::Button::new_with_label(label);
        btn.set_size_request(200, 50);
        btn
    }
}