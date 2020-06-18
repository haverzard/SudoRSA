build:
    @cargo build

setup:
    sudo rm /lib/x86_64-linux-gnu/security/pam_rsa.so
    sudo cp target/debug/libsudorsa.so /lib/x86_64-linux-gnu/security/pam_rsa.so
    sudo chmod 644 /lib/x86_64-linux-gnu/security/pam_rsa.so

cli:
    sudo ./target/debug/cli

gui:
    sudo ./target/debug/gui