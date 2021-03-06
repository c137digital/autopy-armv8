use image::ImageError;
use pyo3::prelude::*;

pub struct FromImageError(ImageError);

impl From<ImageError> for FromImageError {
    fn from(err: ImageError) -> FromImageError {
        FromImageError { 0: err }
    }
}

impl From<FromImageError> for PyErr {
    fn from(err: FromImageError) -> PyErr {
        match err.0 {
            ImageError::DimensionError => {
                pyo3::exceptions::ValueError::py_err(format!("{}", err.0))
            }
            _ => pyo3::exceptions::IOError::py_err(format!("{}", err.0)),
        }
    }
}
