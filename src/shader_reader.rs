use crate::logger::log;
use crate::info_types::InfoTypes::*;

pub fn read(file_bytes: &str, file_name: &str) -> String {
    let logger_path = format!("Loaded shader: {}", file_name);
    log(&logger_path, INFO.literal());
    
    file_bytes.to_string()
}

/*
    let logger_path = "Loaded image: <embedded>".to_string();
    log(&logger_path, INFO.types());

    // Load the image from the byte slice
    let image = image::load(Cursor::new(image_bytes), image::ImageFormat::Png)
        .unwrap()
        .to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);

    // Convert to texture
    let texture = glium::texture::SrgbTexture2d::new(display, image).unwrap();
    texture
*/