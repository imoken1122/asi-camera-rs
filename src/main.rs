#[macro_use]
extern crate log;
extern crate env_logger;
mod camera;
mod libasi;
use camera::*;
mod utils;

fn main() {
        env_logger::init();

        // ASIDevices object mutable variable camera object shoud be mutable
        let mut asi_camera = ASIDevices::new();
        let camera =  asi_camera.get_camera(0);

        camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD ,
                         camera.ctype2caps.get(&libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD).unwrap().MinValue, 0);
        camera.disable_dark_subtract();
        camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);
        println!("{:?}", camera.get_img_type());
        //camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE , 30000, 0);

       let exp_type = libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE;
       let gain_type = libasi::ASI_CONTROL_TYPE_ASI_GAIN;
       let ctl_types = vec![exp_type, gain_type];
        camera.capture_video_frame( Some(ctl_types));
        camera.close();
}

