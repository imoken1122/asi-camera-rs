extern crate libasi_core;
use libasi_core::*;
use pyo3::prelude::*;


#[pyclass]
pub struct  PyCamera(pub Camera);

#[pyclass]
pub struct  PyASIDevices(pub ASIDevices);

#[pyfunction]
fn get_asi_devices(camera_idx:i32) -> PyResult<PyASIDevices> {
    let mut asi_camera = ASIDevices::new();
    return asi_camera;
    //let camera =  asi_camera.get_camera(0).read().unwrap();
}

/// A Python module implemented in Rust.
#[pymodule]
fn libasi_py(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_asi_devices, m)?)?;
    Ok(())
}