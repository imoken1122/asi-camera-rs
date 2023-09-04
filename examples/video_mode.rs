use asi_camera_rs::camera::*;
use asi_camera_rs::libasi;
use std::sync::{Arc};
use std::thread;
use env_logger;
fn main(){
    env_logger::init();
    let mut asi_camera = ASIDevices::new();

    // Camera 1 Setting
    // setting control value of camera 1
    let camera =  asi_camera.get_camera(0);
    
    // camera variable is wraped Arc, it must be unwrapped using read() of write()
    camera.read().unwrap().disable_dark_subtract();
    camera.read().unwrap().set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);

    // auto adjust control types
    let exp_type = libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE;
    let gain_type = libasi::ASI_CONTROL_TYPE_ASI_GAIN;
    let ctl_types = Some(vec![exp_type, gain_type]);
    
    let mut threads = vec![];
    // capture video frmae  using thread
    let c= Arc::clone( &camera);
    threads.push(thread::spawn( move || {
        c.read().unwrap().capture_video_frame(ctl_types);
    }));

    // Set up camera 2 and beyond as in the settings above.
    //
    //



    // sync all threads
    threads.into_iter().for_each(|t|t.join().unwrap());
}