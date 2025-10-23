use image::{DynamicImage, GenericImageView};
use regex::Regex;
use smallvec::*;
use std::io::{Result, Write};
use std::path::Path;
use textwrap::fill;
use unicode_width::UnicodeWidthStr;

const BUFSIZE: usize = 8192;

/// 像素转换模式
#[derive(Debug, Clone, Copy)]
pub enum PixelMode {
    /// 真彩色模式，保持原有颜色
    TrueColor,
    /// 黑白模式，转换为黑白格子
    Monochrome,
    /// 反色模式，黑白颠倒的黑白格子
    Invert,
}

/// 从图片文件创建像素说话效果
///
/// `image_path` 是图片文件的路径
/// `message` 是要显示的消息文本
/// `max_width` 是文本的最大宽度
/// `mode` 是像素转换模式（真彩色或黑白）
/// `writer` 是输出目标
///
/// # Example
///
/// ```rust,no_run
/// use pixel_says::{say_from_image, PixelMode};
/// use std::io::{stdout, BufWriter};
///
/// let stdout = stdout();
/// let message = "Hello from pixels!";
/// let width = 24;
///
/// let writer = BufWriter::new(stdout.lock());
/// say_from_image("test.png", message, width, PixelMode::TrueColor, writer).unwrap();
/// ```
pub fn say_from_image<P, W>(
    image_path: P,
    message: &str,
    max_width: usize,
    mode: PixelMode,
    writer: W,
) -> Result<()>
where
    P: AsRef<Path>,
    W: Write,
{
    // 加载图片
    let img = image::open(image_path).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("无法加载图片: {}", e))
    })?;

    say_from_dynamic_image(img, message, max_width, mode, writer)
}

/// 从 DynamicImage 创建像素说话效果
pub fn say_from_dynamic_image<W>(
    img: DynamicImage,
    message: &str,
    max_width: usize,
    mode: PixelMode,
    mut writer: W,
) -> Result<()>
where
    W: Write,
{
    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();

    // 预处理消息文本
    let input = merge_white_spaces(message);
    let wrapped = fill(input.as_str(), max_width);
    let lines: Vec<&str> = wrapped.lines().collect();
    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // 绘制消息框顶部
    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'_');
    }
    write_buffer.push(b'\n');

    // 绘制消息内容
    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if i == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(b"| ");
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _ in line_len..actual_width {
            write_buffer.push(b' ');
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if i == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(b" |\n");
        }
    }

    // 绘制消息框底部
    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'-');
    }
    write_buffer.push(b'\n');

    // 添加连接线
    write_buffer.extend_from_slice(b"        \\\n");
    write_buffer.extend_from_slice(b"         \\\n");

    // 输出缓冲区内容
    writer.write_all(&write_buffer)?;

    // 转换并输出图片
    convert_image_to_text(img, mode, writer)?;

    Ok(())
}

/// 将图片转换为终端文本
fn convert_image_to_text<W>(img: DynamicImage, mode: PixelMode, writer: W) -> Result<()>
where
    W: Write,
{
    let (width, height) = img.dimensions();
    
    // 限制图片大小，避免输出过大
    let max_size = 80;
    let (new_width, new_height) = if width > max_size || height > max_size {
        let ratio = max_size as f32 / width.max(height) as f32;
        ((width as f32 * ratio) as u32, (height as f32 * ratio) as u32)
    } else {
        (width, height)
    };

    let resized_img = img.resize(new_width, new_height, image::imageops::FilterType::Nearest);

    match mode {
        PixelMode::TrueColor => convert_to_truecolor(&resized_img, writer),
        PixelMode::Monochrome => convert_to_monochrome(&resized_img, writer),
        PixelMode::Invert => convert_to_invert(&resized_img, writer),
    }
}

/// 转换为真彩色输出
fn convert_to_truecolor<W>(img: &DynamicImage, mut writer: W) -> Result<()>
where
    W: Write,
{
    let (width, height) = img.dimensions();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            match pixel {
                image::Rgba([r, g, b, a]) => {
                    // 如果像素是透明的，输出空格
                    if a < 128 {
                        write!(writer, "  ")?;
                    } else {
                        // 使用 ANSI 真彩色转义序列 - 前景色
                        write!(writer, "\x1b[38;2;{};{};{}m██\x1b[0m", r, g, b)?;
                    }
                },
            };
        }
        writeln!(writer)?;
    }
    
    Ok(())
}

/// 转换为黑白模式输出
fn convert_to_monochrome<W>(img: &DynamicImage, mut writer: W) -> Result<()>
where
    W: Write,
{
    let (width, height) = img.dimensions();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let char = match pixel {
                image::Rgba([r, g, b, a]) => {
                    // 如果像素是透明的，输出空格
                    if a < 128 {
                        "  "
                    } else {
                        // 计算亮度 (ITU-R BT.709)
                        let luminance = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8;
                        // 根据亮度选择字符
                        if luminance > 128 { "██" } else { "  " }
                    }
                },
            };
            write!(writer, "{}", char)?;
        }
        writeln!(writer)?;
    }
    
    Ok(())
}

/// 转换为反色模式输出
fn convert_to_invert<W>(img: &DynamicImage, mut writer: W) -> Result<()>
where
    W: Write,
{
    let (width, height) = img.dimensions();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let char = match pixel {
                image::Rgba([r, g, b, a]) => {
                    // 如果像素是透明的，输出空格
                    if a < 128 {
                        "  "
                    } else {
                        // 计算亮度 (ITU-R BT.709)
                        let luminance = (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32) as u8;
                        // 反色：根据亮度选择字符，与monochrome相反
                        if luminance > 128 { "  " } else { "██" }
                    }
                },
            };
            write!(writer, "{}", char)?;
        }
        writeln!(writer)?;
    }
    
    Ok(())
}

/// 兼容原有的 say 函数，使用默认的 Ferris 图案
pub fn say<W>(input: &str, max_width: usize, mut writer: W) -> Result<()>
where
    W: Write,
{
    const MASCOT: &[u8] = br#"
        \
         \
            _~^~^~_
        \) /  o o  \ (/
          '_   -   _'
          / '-----' \
"#;

    let mut write_buffer = SmallVec::<[u8; BUFSIZE]>::new();
    let input = merge_white_spaces(input);
    let wrapped = fill(input.as_str(), max_width);
    let lines: Vec<&str> = wrapped.lines().collect();
    let line_count = lines.len();
    let actual_width = longest_line(&lines);

    // 绘制消息框
    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'_');
    }
    write_buffer.push(b'\n');

    for (i, line) in lines.into_iter().enumerate() {
        if line_count == 1 {
            write_buffer.extend_from_slice(b"< ");
        } else if i == 0 {
            write_buffer.extend_from_slice(b"/ ");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b"\\ ");
        } else {
            write_buffer.extend_from_slice(b"| ");
        }

        let line_len = UnicodeWidthStr::width(line);
        write_buffer.extend_from_slice(line.as_bytes());
        for _ in line_len..actual_width {
            write_buffer.push(b' ');
        }

        if line_count == 1 {
            write_buffer.extend_from_slice(b" >\n");
        } else if i == 0 {
            write_buffer.extend_from_slice(b" \\\n");
        } else if i == line_count - 1 {
            write_buffer.extend_from_slice(b" /\n");
        } else {
            write_buffer.extend_from_slice(b" |\n");
        }
    }

    write_buffer.push(b' ');
    for _ in 0..(actual_width + 2) {
        write_buffer.push(b'-');
    }

    write_buffer.extend_from_slice(MASCOT);
    writer.write_all(&write_buffer)
}

fn longest_line(lines: &[&str]) -> usize {
    lines
        .iter()
        .map(|line| UnicodeWidthStr::width(*line))
        .max()
        .unwrap_or(0)
}

fn merge_white_spaces(input: &str) -> String {
    let re = Regex::new(r"([^\S\r\n])+").unwrap();
    re.replace_all(input, " ").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgba, RgbaImage};

    #[test]
    fn test_transparent_pixels_in_monochrome() {
        // 创建一个 2x2 的测试图片，包含透明和不透明像素
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 255, 255, 255])); // 白色，不透明
        img.put_pixel(1, 0, Rgba([0, 0, 0, 255]));       // 黑色，不透明
        img.put_pixel(0, 1, Rgba([255, 255, 255, 0]));   // 白色，透明
        img.put_pixel(1, 1, Rgba([0, 0, 0, 0]));         // 黑色，透明
        
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let mut output = Vec::new();
        
        convert_to_monochrome(&dynamic_img, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();
        
        // 期望：第一行是"██  "（白色块+黑色空格），第二行是"    "（两个透明像素都是空格）
        assert_eq!(result, "██  \n    \n");
    }

    #[test]
    fn test_transparent_pixels_in_invert() {
        // 创建一个 2x2 的测试图片，包含透明和不透明像素
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 255, 255, 255])); // 白色，不透明
        img.put_pixel(1, 0, Rgba([0, 0, 0, 255]));       // 黑色，不透明
        img.put_pixel(0, 1, Rgba([255, 255, 255, 0]));   // 白色，透明
        img.put_pixel(1, 1, Rgba([0, 0, 0, 0]));         // 黑色，透明
        
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let mut output = Vec::new();
        
        convert_to_invert(&dynamic_img, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();
        
        // 期望：第一行是"  ██"（白色空格+黑色块），第二行是"    "（两个透明像素都是空格）
        assert_eq!(result, "  ██\n    \n");
    }

    #[test]
    fn test_transparent_pixels_in_truecolor() {
        // 创建一个 2x2 的测试图片，包含透明和不透明像素
        let mut img = RgbaImage::new(2, 2);
        img.put_pixel(0, 0, Rgba([255, 0, 0, 255]));     // 红色，不透明
        img.put_pixel(1, 0, Rgba([0, 255, 0, 255]));     // 绿色，不透明
        img.put_pixel(0, 1, Rgba([0, 0, 255, 0]));       // 蓝色，透明
        img.put_pixel(1, 1, Rgba([255, 255, 255, 50]));  // 白色，半透明
        
        let dynamic_img = DynamicImage::ImageRgba8(img);
        let mut output = Vec::new();
        
        convert_to_truecolor(&dynamic_img, &mut output).unwrap();
        let result = String::from_utf8(output).unwrap();
        
        // 期望：第一行有彩色块，第二行两个透明像素都是空格
        assert!(result.contains("\x1b[38;2;255;0;0m██\x1b[0m")); // 红色块
        assert!(result.contains("\x1b[38;2;0;255;0m██\x1b[0m")); // 绿色块
        assert!(result.ends_with("    \n")); // 第二行全是空格
    }
}
