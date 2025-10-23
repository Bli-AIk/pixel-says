use pixel_says::{say_from_image, PixelMode};
use std::env;
use std::io::{stdout, BufWriter};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 3 {
        eprintln!("用法: {} <图片路径> <消息> [--monochrome]", args[0]);
        eprintln!("例子: {} test_pixel.png \"Hello from pixels!\"", args[0]);
        eprintln!("选项: --monochrome 使用黑白模式");
        return;
    }
    
    let image_path = &args[1];
    let message = &args[2];
    let mode = if args.len() > 3 && args[3] == "--monochrome" {
        PixelMode::Monochrome
    } else {
        PixelMode::TrueColor
    };
    
    let stdout = stdout();
    let writer = BufWriter::new(stdout.lock());
    
    if let Err(e) = say_from_image(image_path, message, 40, mode, writer) {
        eprintln!("错误: {}", e);
    }
}