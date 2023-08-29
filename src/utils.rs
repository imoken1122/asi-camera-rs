
use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub enum ROIFormatError {
   InvaildBinSize(u32),
   InvalidWidth(u32),
   InvalidHeight(u32)
}

impl fmt::Display for ROIFormatError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ROIFormatError::InvaildBinSize(bins)=> write!(f, "ROI width or height larger than binned width or height. bins : {}", bins),
            ROIFormatError::IOError(inner) => write!(f, "IO error: {}", inner),
            ROIFormatError::IOError(inner) => write!(f, "IO error: {}", inner),
            ROIFormatError::IOError(inner) => write!(f, "IO error: {}", inner),
        }
    }
}
