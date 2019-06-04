use std::error::Error;
use std::fmt;

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate base64_url;

extern crate hmac;
extern crate sha2;
use sha2::Sha256;
use hmac::{Hmac, Mac};

struct Jwt<'a> {
    b64_signed_part: &'a [u8],
    b64_signature: &'a [u8],
    signature: &'a mut [u8],
}

impl<'a> Jwt<'a> {
    fn check(&self, key: &[u8]) -> Result<(), JwtError> {
        let mut mac = Hmac::<Sha256>::new_varkey(key).unwrap();
        mac.input(self.b64_signed_part);

        let code = mac.result().code();
        let computed_signature = code.as_slice();

        // TODO: extract this outside of the loop
        let b64_signature_vec = self.b64_signature.to_vec();
        let decoded_signature = base64_url::decode(&b64_signature_vec).unwrap();
        let signature = decoded_signature.as_slice();

        if computed_signature.eq(signature) {
            Ok(())
        } else {
            Err(JwtError::InvalidSignature)
        }
    }

    fn new() -> Self {
        Jwt{
            b64_signed_part: &[],
            b64_signature: &[],
            signature: &mut [],
        }
    }

    fn split(&'a mut self, jwt_str: &'a String) -> Result<&'a Self, JwtError> {
        let components: Vec<&str> = jwt_str.rsplitn(2, '.').collect();
        if components.len() != 2 {
            return Err(JwtError::InvalidFormat);
        }
        self.b64_signed_part = components[1].as_bytes();
        self.b64_signature = components[0].as_bytes();

        Ok(self)
    }
}

#[derive(Debug)]
enum JwtError {
    InvalidFormat,
    InvalidSignature,
}

impl fmt::Display for JwtError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            JwtError::InvalidFormat => write!(f, "Invalid jwt format"),
            JwtError::InvalidSignature => write!(f, "Invalid signature"),
        }
    }
}

impl Error for JwtError {
    fn cause(&self) -> Option<&Error> {
        match *self {
            _ => None,
        }
    }
}

fn main() {
    let matches = App::new("jwt-crack")
    .version("0.1")
    .author("Damien Szczyt <damien.szczyt@gmail.com>")
    .about("Brute force jwt secret keys")
    .arg(Arg::with_name("max_length")
        .help("the maximum number of characters")
        .short("l")
        .default_value("6")
        .takes_value(true)
    )
    .arg(Arg::with_name("token")
        .help("the token")
        .index(1)
        .required(true)
    )
    .arg(Arg::with_name("alphabet")
        .help("the alphabet to use")
        .short("a")
        .default_value("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789")
    )
    .get_matches();

    let token = matches.value_of("token").unwrap().to_owned();

    let mut jwt = Jwt::new();
    let ref jwt = jwt.split(&token).unwrap();
    

    let max_length: usize = value_t!(matches, "max_length", usize).unwrap();
    let alphabet = matches.value_of("alphabet").unwrap();
    let alphabet_len = alphabet.len();
    let alphabet_chars = alphabet.as_bytes();

    let mut length = 1;

    'mainloop: while length <= max_length {
        let nb_strs = alphabet_len.pow(length as u32);

        for i in 0..nb_strs {
            let mut current_string = "".to_owned();
            let mut quotient = i;
            for _ in 0..length {
                current_string.push(alphabet_chars[quotient % alphabet_len] as char);
                quotient = quotient / alphabet_len;
            }
            match jwt.check(current_string.as_bytes()) {
                Ok(_) => {
                    println!("Key is {}", current_string);
                    break 'mainloop;
                },
                Err(JwtError::InvalidSignature) => {},
                Err(err) => {
                    eprintln!("ERROR: {}", err);
                    break 'mainloop;
                },
            }
        }
        length += 1;
    }
}
