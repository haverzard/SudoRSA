#[macro_use]
extern crate colour;

mod utility;

use utility::*;
use std::{io::{stdin, stdout, Write}};

fn menu(is_exit: &mut bool) {
    green_ln!("\nPlease choose a menu:");
    green_ln!("1. Generating new private key");
    green_ln!("2. Generating new public key");
    green_ln!("3. Checking public key");
    green_ln!("4. Exit");
    white!("Enter your choice in number: ");
    let _ = stdout().flush();
    let mut line = String::from("");
    stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(res) => {
            let choice : usize = res;
            if choice == 4 {
                *is_exit = true;
            } else if choice == 3 {
                check_pub_key();
            } else if choice == 2 {
                gen_pub_key();
            } else if choice == 1 {
                gen_priv_key();
            } else {
                red_ln!("Your input is out of range!");
            }
        },
        Err(_) => {
            red_ln!("Please enter a valid number!");
        }
    }
}

fn main() {
    yellow_ln!("==============================");
    yellow_ln!("=  SudoRSA CLI by haverzard  =");
    yellow_ln!("==============================\n");
    begin_check();

    let mut is_exit = false;
    // CLI Menus
    while !is_exit {
        menu(&mut is_exit);
    }
    white_ln!("Goodbye!");
}