use super::errors::AppResult;
use image::GenericImageView;
use std::fs::File;

#[derive(Clone, Debug, PartialEq)]
pub struct PNGImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub color_type: png::ColorType,
    pub bit_depth: png::BitDepth,
}

// make PNGImage from image::DynamicImage.
pub fn load_dynamic_image(img: image::DynamicImage) -> AppResult<PNGImage> {
    let (width, height) = img.dimensions();
    let color = img.color();
    let (color_type, bit_depth) = convert_color_png_type(color);

    Ok(PNGImage {
        width: width,
        height: height,
        data: img.to_bytes(),
        color_type: color_type,
        bit_depth: bit_depth,
    })
}

// make PNGImage from png image decoder
pub fn load_png(filepath: &str) -> AppResult<PNGImage> {
    let file = File::open(filepath).unwrap();
    let decoder = png::Decoder::new(file);
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut buf = vec![0; info.buffer_size()];

    // read the frame
    reader.next_frame(&mut buf).unwrap();

    Ok(PNGImage {
        width: info.width,
        height: info.height,
        data: buf,
        color_type: info.color_type,
        bit_depth: info.bit_depth,
    })
}

// cast image::ColorType to png::ColorType
fn convert_color_png_type(ct: image::ColorType) -> (png::ColorType, png::BitDepth) {
    use png::ColorType::*;
    let (ct, bits) = match ct {
        // Not a thing anymore?
        //image::ColorType::Palette(bits) => (Indexed, bits),
        image::ColorType::L8 => (Grayscale, 8),
        image::ColorType::Rgb8 => (RGB, 8),
        image::ColorType::La8 => (GrayscaleAlpha, 8),
        image::ColorType::Rgba8 => (RGBA, 8),
        image::ColorType::Bgra8 => (RGBA, 8),
        image::ColorType::Bgr8 => (RGB, 8),

        image::ColorType::L16 => (Grayscale, 16),
        image::ColorType::Rgb16 => (RGB, 16),
        image::ColorType::La16 => (GrayscaleAlpha, 16),
        image::ColorType::Rgba16 => (RGBA, 16),

        _ => (RGB, 8)
    };
    (ct, png::BitDepth::from_u8(bits).unwrap())
}
