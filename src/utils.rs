use std::error::Error;
use thiserror::Error;
use image;
use image::DynamicImage;

use env_logger;
use ::log::info;
use log::error;
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

pub fn save_img(dyn_img : DynamicImage, extention : &str )  {
    // TODO 
    // supoorte FITS extentino
    let ext = match extention{
        "jpg"=> image::ImageFormat::Jpeg,
        "png"=> image::ImageFormat::Png,
        _ => panic!("Not supported image extension")
        };
    let output_path = generate_filename(extention);
    match dyn_img.save_with_format(output_path.clone(), ext) {
        Ok(()) => info!("Saved image successfly {}", output_path),
        Err(e) => panic!("Failed to save image")
    }
}



fn generate_filename(extension: &str) -> String {
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%Y-%m-%d_%H-%M-%S").to_string();
    format!("./output/{}_{}.{}", formatted_datetime, "output", extension)
}




