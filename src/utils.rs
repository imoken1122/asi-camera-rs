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



pub fn generate_filename(extension: &str) -> String {
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%Y-%m-%d_%H-%M-%S").to_string();
    format!("./output/{}_{}.{}", formatted_datetime, "output", extension)
}




