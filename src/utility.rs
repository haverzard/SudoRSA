use crate::device::path_traversal;
use std::{fs, process::Command, io::{stdin, stdout, Write}};
use rsa::{PublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};
use rand::{thread_rng, Rng, distributions::Alphanumeric, rngs::OsRng};

pub fn gen_priv_key() {
    // Private key generation
    white_ln!("Generating new private key...");
    let _ = Command::new("openssl")
        .arg("genrsa")
        .arg("-out")
        .arg("/rsa_pam.private")
        .arg("1024")
        .output()
        .expect("Something went wrong when using command (maybe openssl not supported)");
    cyan_ln!("Private key generation result: ✅");
}

pub fn gen_pub_key() {
    // List of devices' paths
    let paths = path_traversal();

    // Choose a device
    green_ln!("Choose the device you want to setup:");
    for i in 0..paths.len() {
        green_ln!("{}. {}", i, paths[i]);
    }
    white!("Enter your choice in number: ");
    let _ = stdout().flush();
    let mut line = String::from("");
    stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(res) => {
            // Checking range
            let choice : usize = res;
            if choice < paths.len() {
                // Enter key file name
                let _ = fs::create_dir(paths[choice].to_owned()+"/keys");
                white!("Enter your file name: ");
                let _ = stdout().flush();
                let mut line = String::from("");
                stdin().read_line(&mut line).unwrap();
                
                // Public key generation
                pub_key_generator(paths[choice].to_owned()+"/keys/"+line.trim());
            } else {
                red_ln!("Your input is out of range!");
            }
        },
        Err(_) => {
            red_ln!("Please enter a valid number!");
        }
    };
}

pub fn pub_key_generator(pub_file_path: String) {
    white_ln!("Generating new public key...");
    let _ = Command::new("openssl")
        .arg("rsa")
        .arg("-in")
        .arg("/rsa_pam.private")
        .arg("-out")
        .arg(pub_file_path)
        .arg("-pubout")
        .arg("-outform")
        .arg("PEM")
        .output()
        .expect("Something went wrong when using command (maybe openssl not supported)");
    cyan_ln!("Public key generation result: ✅");
}

pub fn pub_key_checker(pub_file_path: String) -> Option<()> {
    // Read private key
    let file_contents = fs::read_to_string("/rsa_pam.private").expect("Please setup your private key properly.");
    let der_encoded = file_contents
        .lines()
        .filter(|line| !line.starts_with("-"))
        .fold(String::new(), |mut data, line| {
            data.push_str(&line);
            data
        });
    let der_bytes = base64::decode(&der_encoded).expect("Your key format is wrong!");
    let private_key = RSAPrivateKey::from_pkcs1(&der_bytes).expect("Your key format is wrong!");

    // Read public key
    let file_contents = fs::read_to_string(pub_file_path);
    match file_contents {
        Ok(contents) => {
            let der_encoded = contents
                .lines()
                .filter(|line| !line.starts_with("-"))
                .fold(String::new(), |mut data, line| {
                    data.push_str(&line);
                    data
                });
            let der_bytes = base64::decode(&der_encoded);
            match der_bytes {
                Ok(ok_bytes) => {
                    let public_key_res = RSAPublicKey::from_pkcs8(&ok_bytes);
                    match public_key_res {
                        Ok(public_key) => {
                            let random_text : String = thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(20)
                                .collect();
                            let random_text = random_text.as_bytes();
                            
                            let mut rng = OsRng;
                            let padding = PaddingScheme::new_pkcs1v15_encrypt();
                            let enc_data = public_key
                                .encrypt(&mut rng, padding, &random_text[..])
                                .expect("Something wrong happens on the encryption process");
                            let padding = PaddingScheme::new_pkcs1v15_encrypt();
                            match private_key.decrypt(padding, &enc_data) {
                                Ok(res) => {
                                    if res == random_text {
                                        Some(())
                                    } else {
                                        None
                                    }
                                },
                                Err(_) => None
                            }
                        },
                        Err(_) => None
                    }
                },
                Err(_) => None
            }
        },
        Err(_) => None
    }
}

pub fn check_pub_key() {
    // List of devices' paths
    let paths = path_traversal();

    // Choose a device
    green_ln!("Choose the device you want to check:");
    for i in 0..paths.len() {
        green_ln!("{}. {}", i, paths[i]);
    }
    white!("Enter your choice in number: ");
    let _ = stdout().flush();
    let mut line = String::from("");
    stdin().read_line(&mut line).unwrap();
    match line.trim().parse() {
        Ok(res) => {
            // Checking range
            let choice : usize = res;
            if choice < paths.len() {
                // Enter key file name
                let _ = fs::create_dir(paths[choice].to_owned()+"/keys");
                white!("Enter your file name: ");
                let _ = stdout().flush();
                let mut line = String::from("");
                stdin().read_line(&mut line).unwrap();
                
                // Public key generation
                white_ln!("Checking public key...");
                match pub_key_checker(paths[choice].to_owned()+"/keys/"+line.trim()) {
                    Some(_) => {
                        cyan_ln!("Public key check result: ✅");
                    },
                    None => {
                        red_ln!("Public key check result: ❌");
                    }
                }
            } else {
                red_ln!("Your input is out of range!");
            }
        },
        Err(_) => {
            red_ln!("Please enter a valid number!");
        }
    };
}

pub fn bad_priv_key() {
    white_ln!("Private key check result: ❌");
    gen_priv_key();
}

pub fn begin_check() {
    white_ln!("Checking privileges...");
    // Checking read privileges on /etc/shadow
    match fs::File::open("/etc/shadow") {
        Ok(_) => {
            cyan_ln!("Privileges check result: ✅");
        },
        Err(_) => {
            red_ln!("Privileges check result: ❌");
            panic!("Please run this with sudo privileges");
        }
    }

    white_ln!("Checking private key...");
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
                            cyan_ln!("Private key check result: ✅")
                        },
                        Err(_) => bad_priv_key()
                    }
                },
                Err(_) => bad_priv_key()
            }
        },
        Err(_) => bad_priv_key()
    }
}