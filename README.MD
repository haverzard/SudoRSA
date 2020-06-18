# Sudo RSA
![Rust](https://github.com/haverzard/SudoRSA/workflows/Rust/badge.svg)

![SudoRSA-CLI](SudoRSA-CLI.gif)
![SudoRSA-GUI](SudoRSA-GUI.gif)

No need to type your password everytime! Sudo & polkit through RSA using this module.

## Prerequisites
- Rust: https://rustup.rs/
- GTK: https://www.gtk.org/docs/installations/
- PAM: https://zoomadmin.com/HowToInstall/UbuntuPackage/libpam-dev
- OpenSSL: https://www.openssl.org/
- Just: `cargo install just`

## Usage
### Setup your /etc/pam.d
Add this line to your /etc/pam.d/sudo and /etc/pam.d/polkit (above other auth modules):
- `auth sufficient pam_rsa.so`

Example:
```
    #%PAM-1.0

    auth sufficient pam_rsa.so
    ...
```

### Build cli & module
Run `just build`

### Setup your keys
***PLEASE BUILD FIRST***
#### GUI Based
Run `just gui` and it will generate private key automatically. Please create public key to your selected device flash disk.

#### CLI Based
Run `just cli` and it will generate private key automatically. Please create public key to your selected device flash disk.

### Generating your pam_rsa.so
#### Ubuntu and similars
***PLEASE BUILD FIRST***
Run `just setup`

#### Manual Setup (just in case above is not working)
- Run `cargo build`
- Rename and move `target/debug/librsapam.so` to the pam modules folder

## References
- How to Add a PAM Module: https://docs.oracle.com/cd/E53394_01/html/E54787/pam-27.html
- Linux PAM: http://www.linux-pam.org/Linux-PAM-html/Linux-PAM_MWG.html
- PAM SM Rust: https://docs.rs/pamsm/0.3.3/pamsm/
- Getting started with GTK & Rust: https://blog.sb1.io/getting-started-with-rust-and-gtk/