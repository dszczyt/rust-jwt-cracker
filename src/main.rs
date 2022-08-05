use std::fmt;
use std::{error::Error, process::exit};

use clap::Parser;

extern crate base64_url;

use hmac::{Hmac, Mac};
use rust_jwt_cracker::AlphabetBaseGenerator;
use sha2::Sha256;
use tokio::{select, sync::mpsc};

#[derive(Clone)]
struct Jwt {
    b64_signed_part: Vec<u8>,
    b64_signature: Vec<u8>,
    signature: Vec<u8>,
}

impl Jwt {
    fn check(&self, key: Vec<u8>) -> Result<(), JwtError> {
        let mut mac = Hmac::<Sha256>::new_from_slice(&key).unwrap();
        mac.update(&*self.b64_signed_part);

        let result = mac.finalize();

        if *result.into_bytes() == *self.signature {
            Ok(())
        } else {
            Err(JwtError::InvalidSignature)
        }
    }

    fn new() -> Self {
        Jwt {
            b64_signed_part: vec![],
            b64_signature: vec![],
            signature: vec![],
        }
    }

    fn split(mut self, jwt_str: String) -> Result<Self, JwtError> {
        let components: Vec<&str> = jwt_str.rsplitn(2, '.').collect();
        if components.len() != 2 {
            return Err(JwtError::InvalidFormat);
        }
        self.b64_signed_part = components[1].to_owned().into_bytes();
        self.b64_signature = components[0].to_owned().into_bytes();

        let b64_signature_vec = self.b64_signature.to_vec();
        let decoded_signature = base64_url::decode(&b64_signature_vec).unwrap();
        self.signature = decoded_signature;

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

impl Error for JwtError {}

#[derive(Parser, Debug)]
#[clap(author = "Damien Szczyt <damien.szczyt@gmail.com>")]
#[clap(about = "Brute force jwt secret keys")]
#[clap(version = "0.2")]
pub struct Args {
    #[clap(short = 'l', long, value_parser, default_value_t = 6)]
    pub max_length: usize,

    #[clap(long, value_parser)]
    pub token: String,

    #[clap(long, value_parser, default_value_t=String::from("abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789"))]
    pub alphabet: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    let token = args.token;
    let jwt = Jwt::new().split(token).unwrap();

    let max_length = args.max_length;

    let alphabet = args.alphabet.clone();

    let (tx, mut rx) = mpsc::channel(10);
    tokio::spawn(async move {
        for (i, generated_string) in AlphabetBaseGenerator::init(alphabet.chars())
            .with_limit(max_length)
            .enumerate()
        {
            if i % 100000 == 99999 {
                dbg!(&generated_string);
            }
            tx.send(generated_string).await.unwrap();
        }
    });

    let (done_tx, mut done_rx) = mpsc::channel(1);

    loop {
        select! {
            key = rx.recv() => {
                let key = key.unwrap();
                let done_tx = done_tx.clone();
                let jwt = jwt.clone();
                tokio::spawn(async move {
                    match jwt.check(key.as_bytes().to_vec()) {
                        Ok(_) => {
                            println!("Key is {}", key);
                            done_tx.send(()).await.unwrap();
                        }
                        Err(JwtError::InvalidSignature) => {}
                        Err(err) => {
                            eprintln!("ERROR: {}", err);
                            done_tx.send(()).await.unwrap();
                        }
                    }
                });
            },
            _ = done_rx.recv() => {
                exit(0);
            }
        }
    }
}
