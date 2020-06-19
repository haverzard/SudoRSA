build:
    @cargo build --release

setup:
    sudo cp target/release/libsudorsa.so /lib/x86_64-linux-gnu/security/pam_rsa.so
    sudo chmod 644 /lib/x86_64-linux-gnu/security/pam_rsa.so

cli:
    sudo ./target/release/cli

gui:
    sudo ./target/release/gui