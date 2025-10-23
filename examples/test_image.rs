use pixel_says::*;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    println!("Testing traditional Ferris functionality:");
    if let Err(e) = say("Hello, world!", 24, &mut writer) {
        eprintln!("Error: {}", e);
    }

    println!("\nTesting with pixel image:");
    if let Err(e) = say_from_image("test_pixel.png", "Hello with image!", 24, PixelMode::TrueColor, &mut writer) {
        eprintln!("Error: {}", e);
    }
}