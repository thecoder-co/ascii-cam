use anyhow::Result;
use image::{DynamicImage, GenericImageView, GrayImage, imageops::FilterType};
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};
use std::io::{Write, stdout};

fn main() -> Result<()> {
    // Open default camera
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    )?;

    cam.open_stream()?;

    // Precompute ramp once
    let ramp = "@$B%8&WM#*/|()1{}[]?-_+~<>i!lI;:,\"^`'. ";
    let ramp_chars: Vec<char> = ramp.chars().collect();

    loop {
        // Grab frame
        let frame = cam.frame()?;
        let buffer = frame.decode_image::<RgbFormat>()?;

        let img = DynamicImage::ImageRgb8(buffer);

        // Convert to ASCII
        let ascii_frame = convert_image_to_ascii(&img, 500, false, &ramp_chars)?;

        // Clear terminal and print
        print!("\x1B[2J\x1B[H");
        let mut out = stdout();
        out.write_all(ascii_frame.as_bytes())?;
        out.flush()?;
    }
}

fn convert_image_to_ascii(
    img: &DynamicImage,
    target_width: u32,
    invert: bool,
    ramp_chars: &[char],
) -> Result<String> {
    const CHAR_ASPECT: f32 = 0.55;
    let (orig_w, orig_h) = img.dimensions();
    let aspect = orig_h as f32 / orig_w as f32;
    let target_height = (target_width as f32 * aspect * CHAR_ASPECT).round() as u32;

    // Resize and grayscale
    let gray: GrayImage = img
        .resize(target_width, target_height, FilterType::Nearest) // Faster than Lanczos3
        .grayscale()
        .into();

    let ramp_len = ramp_chars.len() as f32;
    let mut out = String::with_capacity((target_width * target_height + target_height) as usize);

    for y in 0..gray.height() {
        for x in 0..gray.width() {
            let lum = gray.get_pixel(x, y).0[0] as f32 / 255.0;
            let idx = if invert {
                ((1.0 - lum) * (ramp_len - 1.0)).round() as usize
            } else {
                (lum * (ramp_len - 1.0)).round() as usize
            };
            out.push(ramp_chars[idx]);
        }
        out.push('\n');
    }

    Ok(out)
}
