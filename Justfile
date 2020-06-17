build:
    @cargo build

ubuntu:
    sudo rm /lib/x86_64-linux-gnu/security/pam_rsa.so
    sudo cp target/debug/librsapam.so /lib/x86_64-linux-gnu/security/pam_rsa.so

cli:
    sudo ./target/debug/cli

ui:
    sudo ./target/debug/rsapam