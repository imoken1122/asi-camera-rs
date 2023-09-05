
use chrono::prelude::*;

use crate::camera::BufType;
pub fn generate_filename(extension: &str) -> String {
    let current_datetime = Local::now();
    let formatted_datetime = current_datetime.format("%Y-%m-%d_%H-%M-%S.%f").to_string();
    format!("./output/{}_{}.{}", formatted_datetime, "output", extension)
}

