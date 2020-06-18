#[macro_use]
extern crate pamsm;
extern crate rand;
extern crate base64;

mod device;

use std::{fs, str, process::Command};
use rsa::{PublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};
use rand::{thread_rng, Rng, distributions::Alphanumeric, rngs::OsRng};
use pamsm::{Pam, PamServiceModule, PamError, PamFlag};

struct PamRsa;

impl PamServiceModule for PamRsa {
    fn authenticate(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        println!("Trying authenticate using SudoRSA module...");
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
        let mut success = false;
        let paths = device::path_traversal();
        for path in paths {
            let output = Command::new("ls")
                .arg(path.to_owned()+"/keys")
                .output()
                .expect("Something went wrong when using command (maybe lsblk not supported)");

            let keys : Vec<&str> = str::from_utf8(&output.stdout).unwrap().split('\n').filter(|s| s != &"" && s != &"/").collect();
            for key in keys {
                let file_contents = fs::read_to_string(path.to_owned()+"/keys/"+key);
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
                                                    success = true;
                                                }
                                            },
                                            Err(_) => ()
                                        }
                                    },
                                    Err(_) => ()
                                }
                            },
                            Err(_) => ()
                        }
                    },
                    Err(_) => ()
                };
            }
        }
        
        if success {
            println!("Success!");
            PamError::SUCCESS
        } else {
            println!("Failed...");
            PamError::AUTH_ERR
        }
    }
    fn open_session(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        PamError::SUCCESS
    }
    fn close_session(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        PamError::SUCCESS
    }
    fn setcred(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        PamError::SUCCESS
    }
    fn acct_mgmt(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        PamError::SUCCESS
    }
    fn chauthtok(_pamh: Pam, _flags: PamFlag, _args: Vec<String>) -> PamError {
        PamError::SUCCESS
    }
}

pam_module!(PamRsa);