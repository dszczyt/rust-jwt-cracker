use std::error::Error;
use std::fmt;
use std::sync::mpsc::{Sender, SyncSender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::sync::{Arc, Mutex};

#[macro_use]
extern crate clap;
use clap::{Arg, App};

extern crate base64_url;

extern crate hmac;
extern crate sha2;
use sha2::Sha256;
use hmac::{Hmac, Mac};

static NTHREADS: usize = 64;

#[derive(Clone)]
struct Jwt {
    b64_signed_part: Vec<u8>,
    b64_signature: Vec<u8>,
    signature: Vec<u8>,
}

impl Jwt {
    fn check(&self, key: Vec<u8>) -> Result<(), JwtError> {
        let mut mac = Hmac::<Sha256>::new_varkey(&*key).unwrap();
        mac.input(&*self.b64_signed_part);

        if &*mac.result().code() == &*self.signature {
            Ok(())
        } else {
            Err(JwtError::InvalidSignature)
        }
    }

    fn new() -> Self {
        Jwt{
            b64_signed_part: vec!(),
            b64_signature: vec!(),
            signature:vec!(),
        }
    }

    fn split(mut self, jwt_str: String) -> Result<Self, JwtError> {
        let components: Vec<&str> = jwt_str.rsplitn(2, '.').collect();
        if components.len() != 2 {
            return Err(JwtError::InvalidFormat);
        }
        self.b64_signed_part = components[1].to_owned().into_bytes();
        self.b64_signature = components[0].to_owned().into_bytes();

        // TODO: extract this outside of the loop
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

    let token = value_t!(matches, "token", String).unwrap();
    let jwt = Jwt::new().split(token).unwrap();
    

    let max_length = value_t!(matches, "max_length", usize).unwrap();

    let mut length = 1;

    let (key_tx, key_rx): (SyncSender<Vec<u8>>, Receiver<Vec<u8>>) = mpsc::sync_channel(NTHREADS*2);
    let mut children = Vec::with_capacity(NTHREADS);
    let (response_tx, response_rx): (Sender<String>, Receiver<String>) = mpsc::channel(); 

    let multi_rx = Arc::new(Mutex::new(key_rx));

    for thread_number in 0..NTHREADS {
        let mutex_rx = multi_rx.clone();
        let jwt = jwt.clone();
        let response_tx = response_tx.clone();
        let child = thread::spawn(move || {
            loop {
                let x = mutex_rx.lock();
                let z = x.unwrap();
                match z.recv() {
                    Ok(current_string) => {
                        let key = String::from_utf8(current_string.to_vec()).unwrap();
                        //dbg!(key);
                        println!("{} Starting {}", thread_number, key);
                        match jwt.check(key.as_bytes().to_vec()) {
                            Ok(_) => {
                                response_tx.send(key).unwrap();
                                break;
                            },
                            Err(JwtError::InvalidSignature) => {},
                            Err(err) => {
                                eprintln!("ERROR: {}", err);
                                break;
                            },
                        }
                        println!("{} Ending {}", thread_number, key);
                    },
                    Err(_) => break
                }
            }
        });
        children.push(child);
    }

    let alphabet = matches.value_of("alphabet").unwrap();
    let alphabet_len = alphabet.len();
    let alphabet_chars = alphabet.as_bytes();

    'mainloop: while length <= max_length {
        let nb_strs = alphabet_len.pow(length as u32);
        let mut current_string = Box::new(vec![0; length]);

        for i in 0..nb_strs {
            let mut quotient = i;
            for l in 0..length {
                current_string[l] = alphabet_chars[quotient % alphabet_len];
                quotient = quotient / alphabet_len;
            }
            key_tx.send(current_string.to_vec()).unwrap();
            let response = response_rx.try_recv();
            match response {
                Ok(response) => {
                    println!("Secret is \"{}\"", response);
                    break 'mainloop;
                },
                _ => {}
            }
        }
        length += 1;
    }

    for child in children {
        child.join().unwrap();
    }

}
