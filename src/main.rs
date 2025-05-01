use std::process::ExitCode;

use anyhow::Result;
use anyhow::anyhow;
use clap::Parser;
use icy_sixel::DiffusionMethod;
use icy_sixel::MethodForLargest;
use icy_sixel::MethodForRep;
use icy_sixel::PixelFormat;
use icy_sixel::Quality;
use icy_sixel::sixel_string;
use image::GenericImageView;
use image::ImageReader;

/// Cat images using sixel
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None, arg_required_else_help = true)]
struct Args {
    /// The input image file
    input: String,
}

fn run() -> Result<()> {
    let args = Args::parse();

    let img = ImageReader::open(args.input)?.decode()?;

    let rgba_img = img.to_rgba8();

    let bytes: &[u8] = rgba_img.as_raw();

    let (width, height) = img.dimensions();

    let sixel_result = sixel_string(
        bytes,
        width as i32,
        height as i32,
        PixelFormat::RGBA8888,
        DiffusionMethod::Auto,
        MethodForLargest::Auto,
        MethodForRep::Auto,
        Quality::AUTO,
    )
    .map_err(|e| anyhow!("{}", e))?;

    println!("{}", sixel_result);

    Ok(())
}

fn main() -> ExitCode {
    match run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("Error: {:?}", e);
            ExitCode::FAILURE
        }
    }
}
