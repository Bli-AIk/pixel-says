use clap::{command, value_parser, Arg, ArgAction};
use pixel_says::*;
use std::{
    error::Error,
    fs,
    io::{stderr, stdin, stdout, BufWriter, Read, Write},
    path::PathBuf,
    process::exit,
    str,
};

// Constants used for err messages
const INPUT: &str = "Failed to read input to the program";
const STDOUT: &str = "Failed to write stdout";
const STDERR: &str = "Failed to write stderr";

fn main() {
    if let Err(ref e) = run() {
        let stderr = &mut stderr();

        writeln!(stderr, "error: {}", e).expect(STDERR);

        exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let args = command!("Pixel Says")
        .about("Prints out input text with a pixel image")
        .arg(
            Arg::new("FILES")
                .long("files")
                .short('f')
                .help("Set the input files to use")
                .action(ArgAction::Append)
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(
            Arg::new("WIDTH")
                .long("width")
                .short('w')
                .help("Set the width of the text box")
                .default_value("40")
                .value_parser(value_parser!(usize)),
        )
        .arg(
            Arg::new("IMAGE")
                .long("image")
                .short('i')
                .help("Path to the pixel image file")
                .value_parser(value_parser!(PathBuf)),
        )
        .arg(Arg::new("TEXT").action(ArgAction::Append))
        .get_matches();

    let width = *args.get_one::<usize>("WIDTH").unwrap();

    let mut stdin = stdin();
    let stdout = stdout();

    let mut writer = BufWriter::new(stdout.lock());

    // Get the image path if provided
    let image_path = args.get_one::<PathBuf>("IMAGE");

    if let Some(files) = args.get_many::<PathBuf>("FILES") {
        // Read in files and say them with the pixel image
        for f in files {
            let content = fs::read_to_string(f).map_err(|_| INPUT)?;
            if let Some(img_path) = image_path {
                say(&content, width, img_path, &mut writer).map_err(|e| format!("Failed to display with image: {}", e))?;
            } else {
                say_simple(&content, width, &mut writer).map_err(|_| STDOUT)?;
            }
        }
        Ok(())
    } else if let Some(other_args) = args.get_many::<String>("TEXT") {
        let text = other_args
            .map(String::as_str)
            .collect::<Vec<&str>>()
            .join(" ");
        if let Some(img_path) = image_path {
            say(&text, width, img_path, &mut writer).map_err(|e| format!("Failed to display with image: {}", e))?;
        } else {
            say_simple(&text, width, &mut writer).map_err(|_| STDOUT)?;
        }
        Ok(())
    } else {
        let mut input = String::new();
        stdin.read_to_string(&mut input).map_err(|_| INPUT)?;
        if let Some(img_path) = image_path {
            say(&input, width, img_path, &mut writer).map_err(|e| format!("Failed to display with image: {}", e))?;
        } else {
            say_simple(&input, width, &mut writer).map_err(|_| STDOUT)?;
        }
        Ok(())
    }
}
