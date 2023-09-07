use qrcode::{render::unicode, QrCode};
use std::env;

fn main() {
    let url = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please provide a url")
    };
    match QrCode::new(url.as_bytes()) {
        Ok(bytes) => {
            let qrcode = bytes.render::<unicode::Dense1x2>().build();
            println!("{}", qrcode);
        }
        Err(err) => println!("{}", err),
    }
}
