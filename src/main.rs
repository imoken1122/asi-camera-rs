mod camera;
mod libasi;
use camera::*;
mod utils;
fn main() {
        // ASIDevices object mutable variable, because camera object in ASIDevices shoud be mutable
        let mut asi_camera = ASIDevices::new();
        let camera =  asi_camera.get_camera(0);
        for ctl in camera.ctlcaps_mapper.iter() {
                println!("{:?} : {:?}", ctl.0, ctl.1.DefaultValue);
        }

        camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD as i32,
                         camera.ctlcaps_mapper.get(&libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD).unwrap().MinValue, 0);
        camera.disable_dark_subtract();
        camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RAW8);
        println!("{:?}", camera.get_img_type());
        camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE as i32, 30000, 0);
        println!("{:?}", camera.get_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE as i32));
        camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);
        println!("{:?}", camera.get_img_type());
        camera.capture();
        camera.close();
}

