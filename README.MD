# Sudo RSA
No need to type your password everytime! Sudo & polkit through RSA using this module.

## Prerequisites
- Rust: https://rustup.rs/
- Just: `cargo install just`

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

### Setup your keys
Run `just cli` and it will generate private key automatically. Please create public key to your selected device flash disk.

### Generating your pam_rsa.so
#### Ubuntu and similars
Run `just ubuntu`

#### Manual Setup
- Run `cargo build`
- Rename and move `target/debug/librsapam.so` to the pam modules folder