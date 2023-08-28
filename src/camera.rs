
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use crate::{libasi as libasi, camera};


pub struct Camera {
    // id
    // info

}

pub trait CameraControl{
    // main control
    // parameter control
    // camera setting control

}

pub trait ParameterControl{

}
pub trait SettingControl{

}

impl Camera{
    pub fn init(camera_idx : i32){
        // get number of camera
        let num_camera= libasi::get_num_of_connected_cameras();
        println!("num of camera devices {}", num_camera);

        // init caemra_info 
        let mut camera_info = libasi::_ASI_CAMERA_INFO::new();

        // assigning connected camera_i property to camera_info
        libasi::get_camera_prop(&mut camera_info, camera_idx);
        println!("{:?}",camera_info);


    }


}





mod test{
    use super::*;

    #[test]
    fn test_camera(){
        let camera = Camera{};
    }

}