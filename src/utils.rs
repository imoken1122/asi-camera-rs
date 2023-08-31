use std::error::Error;
use thiserror::Error;
use image;
use image::DynamicImage;

use chrono::prelude::*;

use crate::libasi;
use crate::camera::*;
#[derive(Error,Debug)]
pub enum ROIFormatError {
    #[error("Invailed bin size ")]
   InvaildBinSize,
    #[error("Invailed bin size ")]
   InvalidWidth,
    #[error("Invailed bin size ")]
   InvalidHeight
}
pub fn create_buffer(buf_size: BufSize) -> BufType {
    vec![0; buf_size as usize]
}

pub fn buf_to_img(buffer: &[u8],width: u32,height: u32, img_type : libasi::ASIImgType) -> DynamicImage {
     // convert to image by image type (RAW8,RAW8,RGB24,Y8)
    let dyn_img  = match img_type{
        libasi::ASI_IMG_TYPE_ASI_IMG_RGB24=> DynamicImage::ImageRgb8(image::RgbImage::from_raw(width, height, buffer.to_vec()).unwrap()),
        _ => panic!("Invalid image type")
    };
    dyn_img

}
pub fn save_img(dyn_img : DynamicImage, extention : &str ) -> Result<(), image::ImageError> {
    let ext = match extention{
        "jpg"=> image::ImageFormat::Jpeg,
        "png"=> image::ImageFormat::Png,
        _ => panic!("Not supported image extension")
        };
    let output_path = generate_filename(extention);
    dyn_img.save_with_format(output_path, ext)
}



fn generate_filename(extension: &str) -> String {
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%Y-%m-%d_%H-%M-%S").to_string();
    format!("{}_{}.{}", formatted_datetime, "output", extension)
}




