# Sudo RSA
![Rust](https://github.com/haverzard/SudoRSA/workflows/Rust/badge.svg)

No need to type your password everytime! Sudo & polkit through RSA using this module.

## Prerequisites
- Rust: https://rustup.rs/
- Just: `cargo install just`
- libpam

## Generating keys
### Private
`openssl genrsa -out <private_key_file_path> 1024` (you can change the bits too if you want)

### Public
`openssl rsa -in <private_key_file_path> -out <public_key_file_path> -pubout -outform PEM`

## Usage
### Setup your /etc/pam.d
Add this line on the top of your /etc/pam.d/sudo and /etc/pam.d/polkit (the top row after comment):
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
Run `just cli` and it will generate private key automatically. Please create public key to your selected device flash disk.

### Generating your pam_rsa.so
#### Ubuntu and similars
***PLEASE BUILD FIRST***
Run `just ubuntu`

#### Manual Setup (just in case above is not working)
- Run `cargo build`
- Rename and move `target/debug/librsapam.so` to the pam modules folder

## References
- How to Add a PAM Module: https://docs.oracle.com/cd/E53394_01/html/E54787/pam-27.html
- Linux PAM: http://www.linux-pam.org/Linux-PAM-html/Linux-PAM_MWG.html
- PAM SM Rust: https://docs.rs/pamsm/0.3.3/pamsm/