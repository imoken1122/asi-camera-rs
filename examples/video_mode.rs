use asi_camera_rs::camera::*;
use asi_camera_rs::libasi;
use std::thread;
use env_logger;
fn main(){
    env_logger::init();
    let asi_camera = ASIDevices::new();

    // Camera 1 Setting
    let camera =  asi_camera.get_camera(0);
    
    // Which image type to use (rgb24,raw16,raw8)
    camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);

    // auto adjust control types
    let exp_type = libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE;
    let gain_type = libasi::ASI_CONTROL_TYPE_ASI_GAIN;
    let ctl_types = Some(vec![exp_type, gain_type]);
    
    // buffer thread
    let mut threads = vec![];

    // capture video frmae  using threads
    threads.push(thread::spawn( move || {
        camera.capture_video_frame(ctl_types);
    }));

    // Set up camera 2 and beyond as in the settings above. and add therad
    //
    //



    // sync all threads
    threads.into_iter().for_each(|t|t.join().unwrap());
}