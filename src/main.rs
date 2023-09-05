use qrcode::{render::svg, QrCode};
use std::env;

fn main() {
    let url = if env::args().count() == 2 {
        env::args().nth(1).unwrap()
    } else {
        panic!("Please enter a url")
    };
    match QrCode::new(url.as_bytes()) {
        Ok(bytes) => {
            let qrcode = bytes
                .render()
                .dark_color(svg::Color("#800000"))
                .light_color(svg::Color("#ffff80"))
                .min_dimensions(200, 200)
                .build();
            println!("{}", qrcode);
        }
        Err(err) => println!("{}", err),
    }
}
