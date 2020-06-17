use std::{fs, process::Command, str, io::{stdin, stdout, Write}};
use rsa::{PublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};

fn gen_priv_key() {
    println!("Generating new private key...");
    let _ = Command::new("openssl")
        .arg("genrsa")
        .arg("-out")
        .arg("/rsa_pam.private")
        .arg("1024")
        .output()
        .expect("Something went wrong when using command (maybe lsblk not supported)");
    println!("Private key generation result: ✅");
}

fn gen_pub_key() {
    let output = Command::new("lsblk")
        .arg("--output")
        .arg("MOUNTPOINT")
        .output()
        .expect("Something went wrong when using command (maybe lsblk not supported)");
    let paths : Vec<&str> = str::from_utf8(&output.stdout).unwrap().split('\n').filter(|s| s != &"" && s != &"/").collect();
    println!("Choose the device you want to setup:");
    for i in 1..paths.len() {
        println!("{}. {}", i, paths[i]);
    }
    print!("Enter your choice in number: ");
    let _ = stdout().flush();
    let mut line = String::from("");
    stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(res) => {
            let choice : usize = res;
            if choice > 0 && choice < paths.len() {
                let _ = fs::create_dir(paths[choice].to_owned()+"/keys");
                print!("Enter your file name: ");
                let _ = stdout().flush();
                let mut line = String::from("");
                stdin().read_line(&mut line).unwrap();
                let _ = Command::new("openssl")
                    .arg("rsa")
                    .arg("-in")
                    .arg("/rsa_pam.private")
                    .arg("-out")
                    .arg(paths[choice].to_owned()+"/keys/"+line.trim())
                    .arg("-pubout")
                    .arg("-outform")
                    .arg("PEM")
                    .output()
                    .expect("Something went wrong when using command (maybe lsblk not supported)");
            } else {
                println!("Your number is out of range!");
            }
        },
        Err(_) => {
            println!("Please enter a valid number!");
        }
    };
}

fn bad_priv_key() {
    println!("Private key check result: ❌");
    gen_priv_key();
}

fn main() {
    println!("Checking privileges...");
    match fs::File::open("/etc/shadow") {
        Ok(_) => {
            println!("Privileges check result: ✅");
        },
        Err(_) => {
            println!("Privileges check result: ❌");
            panic!("Please run this with sudo privileges");
        }
    }
    println!("Checking private key...");
    // Read private key
    match fs::read_to_string("/rsa_pam.private") {
        Ok(res) => {
            let file_contents = res;
            let der_encoded = file_contents
                .lines()
                .filter(|line| !line.starts_with("-"))
                .fold(String::new(), |mut data, line| {
                    data.push_str(&line);
                    data
                });
            match base64::decode(&der_encoded) {
                Ok(res) => {
                    let der_bytes = res;
                    match RSAPrivateKey::from_pkcs1(&der_bytes) {
                        Ok(_) => {
                            println!("Private key check result: ✅")
                        },
                        Err(_) => bad_priv_key()
                    }
                },
                Err(_) => bad_priv_key()
            }
        },
        Err(_) => bad_priv_key()
    }
    println!("Please choose a menu:");
    println!("1. Generating new private key");
    println!("2. Generating new public key");
    println!("3. Checking public key");
    print!("Enter your choice in number: ");
    let _ = stdout().flush();
    let mut line = String::from("");
    stdin().read_line(&mut line).unwrap();
    
}