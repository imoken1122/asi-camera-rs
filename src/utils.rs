
use std::error::Error;
use std::fmt;
use thiserror::Error;
use image;
use std::fs::File;
use image::DynamicImage;
use std::time::{SystemTime, UNIX_EPOCH};
#[derive(Error,Debug)]
pub enum ROIFormatError {
    #[error("Invailed bin size ")]
   InvaildBinSize,
    #[error("Invailed bin size ")]
   InvalidWidth,
    #[error("Invailed bin size ")]
   InvalidHeight
}



pub fn write_image_to_png(buffer: &[u8], width: u32, height: u32, ) -> Result<(), image::ImageError> {
    let output_path = generate_filename("png".to_string());
    let dynamic_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(width, height, buffer.to_vec()).unwrap());
    dynamic_image.save(output_path)
}

pub fn write_image_to_jpeg(buffer: &[u8], width: u32, height: u32, ) -> Result<(), image::ImageError> {
    let output_path = generate_filename("jpg".to_string());
    let dynamic_image = DynamicImage::ImageRgb8(image::RgbImage::from_raw(width, height, buffer.to_vec()).unwrap());
    dynamic_image.save_with_format(output_path, image::ImageFormat::Jpeg)
}


fn generate_filename(ext : String) -> String {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let formatted_time = since_epoch.as_secs();
    format!("output/image_{}.{}", formatted_time,ext)
}




