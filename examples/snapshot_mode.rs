

use asi_camera_rs::camera::*;
use asi_camera_rs::libasi;
fn main(){
    env_logger::init();
    let  asi_camera = ASIDevices::new();
    let camera =  asi_camera.get_camera(0);

    camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD ,
                        camera.ctype2caps.get(&libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD).unwrap().MinValue, 0);
    camera.disable_dark_subtract();
    camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE , 30, 0);
    //camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);
    camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RAW16 );
    camera.snapshot();
    camera.close();


}