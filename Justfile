ubuntu:
    @cargo build
    sudo rm /lib/x86_64-linux-gnu/security/pam_rsa.so
    sudo cp target/debug/librsapam.so /lib/x86_64-linux-gnu/security/pam_rsa.so

cli:
    cargo build
    sudo ./target/debug/rsapam