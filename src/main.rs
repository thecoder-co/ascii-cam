use anyhow::Result;
use image::{DynamicImage, GenericImageView, GrayImage, imageops::FilterType};
use nokhwa::{
    Camera,
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
};
use std::io::{Write, stdout};
use std::time::Instant;

fn main() -> Result<()> {
    // Open default camera
    let mut cam = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    )?;

    cam.open_stream()?;

    // Precompute ramp once
    let ramp = "@#%WX8B&$M0QOZ*+i!;:,'. ";
    let ramp_chars: Vec<char> = ramp.chars().collect();

    let mut last_frame_time = Instant::now();
    let mut fps = 0.0;
    loop {
        let now = Instant::now();
        let delta = now.duration_since(last_frame_time).as_secs_f32();
        last_frame_time = now;

        fps = 1.0 / delta;

        let frame = cam.frame()?;
        let buffer = frame.decode_image::<RgbFormat>()?;

        let img = DynamicImage::ImageRgb8(buffer);

        let ascii_frame = convert_image_to_ascii(&img, 1000, true, &ramp_chars)?;

        print!("\x1B[2J\x1B[H");
        let mut out = stdout();
        writeln!(out, "FPS: {:.1}", fps)?;
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

    let gray: GrayImage = img
        .resize(target_width, target_height, FilterType::Nearest)
        .grayscale()
        .into();

    let ramp_len = ramp_chars.len() as f32;
    let mut out = String::with_capacity((target_width * target_height + target_height) as usize);

    let pixels = gray.as_raw();
    let width = gray.width() as usize;

    for row in pixels.chunks_exact(width) {
        for &lum in row {
            let lum_norm = lum as f32 / 255.0;
            let idx = if invert {
                ((1.0 - lum_norm) * (ramp_len - 1.0)).round() as usize
            } else {
                (lum_norm * (ramp_len - 1.0)).round() as usize
            };
            out.push(ramp_chars[idx]);
        }
        out.push('\n');
    }

    Ok(out)
}
