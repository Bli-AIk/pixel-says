# Pixel Says / 像素说话

[![Rust](https://img.shields.io/badge/rust-2021-orange.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-MIT)

> 🎨 A Rust library that converts pixel images into terminal cowsay with true color or monochrome support!  
> 🎨 一个将像素图转换为终端 cowsay 的 Rust 库，支持真彩色和黑白模式！

[English](#english) | [中文](#中文)

---

## English

### 🎯 About

`pixel-says` is a fun library that transforms any pixel image into terminal text art that can "speak"! It's based on the excellent [`ferris-says`](https://github.com/rust-lang/ferris-says) library and was developed entirely through **AI-assisted vibe coding** - just for the joy of creating something cool! 🤖✨

### ✨ Features

- 🌈 **True Color Mode**: Preserves original image colors using ANSI true color escape sequences
- ⚫ **Monochrome Mode**: Converts images to black and white blocks based on pixel luminance
- 🔄 **Invert Mode**: Converts images to inverted black and white blocks (reverse of monochrome)
- 📏 **Auto Scaling**: Automatically resizes large images to fit terminal display
- 🔄 **Backward Compatible**: Maintains compatibility with original `ferris-says` functionality
- 🖼️ **Multiple Formats**: Supports PNG, JPEG, GIF, BMP, ICO, TIFF, WebP, AVIF

### 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
pixel-says = "0.1.0"
```

### 📖 Usage

#### Pixel Image Mode

```rust
use pixel_says::{say_from_image, PixelMode};
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    // True color mode
    say_from_image("image.png", "Hello from pixels!", 40, PixelMode::TrueColor, &mut writer).unwrap();
    
    // Monochrome mode
    say_from_image("image.png", "Hello in B&W!", 40, PixelMode::Monochrome, &mut writer).unwrap();
    
    // Invert mode  
    say_from_image("image.png", "Hello inverted!", 40, PixelMode::Invert, &mut writer).unwrap();
}
```

#### Classic Ferris Mode

```rust
use pixel_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say("Hello fellow Rustaceans!", 24, &mut writer).unwrap();
}
```

#### Command Line Usage

```bash
# True color mode (default)
cargo run -- image.png "Hello from pixels!"

# Monochrome mode
cargo run -- image.png "Hello in monochrome!" --monochrome

# Invert mode
cargo run -- image.png "Hello inverted!" --invert
```

### 🛠️ Build Requirements

- Stable Rust compiler (2021 edition)

### 🎨 Example Output

When you run the pixel mode, you'll see your image converted to colorful terminal blocks that "speak" your message!

### 📝 Credits & Background

This project is a playful fork of [`ferris-says`](https://github.com/rust-lang/ferris-says) by the Rust community. The entire development process was done through **AI-assisted programming** (vibe coding) - demonstrating how AI can help create fun and functional software projects!

- Original Ferris ASCII art by [@Diggsey](https://www.reddit.com/r/rust/comments/52vb6y/animated_ferris_the_rustacean/d7phkyh/)
- Built with love and AI assistance 🤖❤️

---

## 中文

### 🎯 关于项目

`pixel-says` 是一个有趣的库，可以将任何像素图转换为能够"说话"的终端文本艺术！它基于优秀的 [`ferris-says`](https://github.com/rust-lang/ferris-says) 库开发，整个开发过程完全通过 **AI 辅助编程（vibe coding）** 完成 - 纯粹为了创造有趣的东西而做的！🤖✨

### ✨ 特色功能

- 🌈 **真彩色模式**: 使用 ANSI 真彩色转义序列保持原始图片颜色
- ⚫ **黑白模式**: 根据像素亮度将图片转换为黑白格子
- 🔄 **反色模式**: 将图片转换为反色黑白格子（黑白颠倒）
- 📏 **自动缩放**: 自动调整大图片尺寸以适配终端显示
- 🔄 **向后兼容**: 保持与原始 `ferris-says` 功能的兼容性
- 🖼️ **多格式支持**: 支持 PNG、JPEG、GIF、BMP、ICO、TIFF、WebP、AVIF

### 🚀 快速开始

在 `Cargo.toml` 中添加：

```toml
[dependencies]
pixel-says = "0.1.0"
```

### 📖 使用方法

#### 像素图模式

```rust
use pixel_says::{say_from_image, PixelMode};
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    
    // 真彩色模式
    say_from_image("image.png", "来自像素的问候！", 40, PixelMode::TrueColor, &mut writer).unwrap();
    
    // 黑白模式
    say_from_image("image.png", "黑白世界！", 40, PixelMode::Monochrome, &mut writer).unwrap();
    
    // 反色模式
    say_from_image("image.png", "反色世界！", 40, PixelMode::Invert, &mut writer).unwrap();
}
```

#### 经典 Ferris 模式

```rust
use pixel_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let mut writer = BufWriter::new(stdout.lock());
    say("你好，Rust 开发者们！", 24, &mut writer).unwrap();
}
```

#### 命令行使用

```bash
# 真彩色模式（默认）
cargo run -- image.png "来自像素的问候！"

# 黑白模式
cargo run -- image.png "黑白世界！" --monochrome

# 反色模式  
cargo run -- image.png "反色世界！" --invert
```

### 🛠️ 构建要求

- 稳定版 Rust 编译器（2021 版本）

### 🎨 示例输出

当你运行像素模式时，你会看到你的图片被转换为彩色的终端块，并"说出"你的消息！

### 📝 致谢与背景

这个项目是 Rust 社区的 [`ferris-says`](https://github.com/rust-lang/ferris-says) 的一个有趣分支。整个开发过程都是通过 **AI 辅助编程** 完成的 - 展示了 AI 如何帮助创建有趣且实用的软件项目！

- 原始 Ferris ASCII 艺术由 [@Diggsey](https://www.reddit.com/r/rust/comments/52vb6y/animated_ferris_the_rustacean/d7phkyh/) 创作
- 用爱与 AI 辅助构建 🤖❤️

---

## 📄 License / 许可证

Licensed under either of / 双重许可：

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option / 任选其一。

## 🤝 Contributing / 贡献

This project was created for fun through AI-assisted programming! Feel free to fork, experiment, and create your own vibe-coded variations! 

这个项目是通过 AI 辅助编程为了乐趣而创建的！欢迎 fork、实验，并创建你自己的 vibe coding 变体！

---

<div align="center">
<i>Made with 🤖 AI assistance and ❤️ for the Rust community</i><br>
<i>使用 🤖 AI 辅助和对 Rust 社区的 ❤️ 制作</i>
</div>