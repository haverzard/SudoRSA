extern crate pamsm;
extern crate rand;
extern crate base64;

use std::fs;
use rsa::{PublicKey, PaddingScheme, RSAPrivateKey, RSAPublicKey};
use rand::{thread_rng, Rng, distributions::Alphanumeric, rngs::OsRng};
use pamsm::pam_raw::{PamError, PamFlag};
use pamsm::{Pam, PamServiceModule};

struct RSAPam;

impl PamServiceModule for RSAPam {
    fn authenticate(self: &Self, pamh: Pam, _flags: PamFlag, args: Vec<String>) -> PamError {
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

        let file_contents = fs::read_to_string("/rsa_pam.public");
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
                        let public_key_res = RSAPublicKey::from_pkcs1(&ok_bytes);
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
                                let dec_data = private_key
                                    .decrypt(padding, &enc_data)
                                    .expect("Something wrong happens on the decryption process");

                                if dec_data == random_text {
                                    PamError::SUCCESS
                                } else {
                                    PamError::AUTH_ERR
                                }
                            },
                            Err(_) => PamError::AUTH_ERR
                        }
                    },
                    Err(_) => PamError::AUTH_ERR
                }
            },
            Err(_) => PamError::AUTH_ERR
        }
        
    }
}
