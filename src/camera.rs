
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{libasi::{self as libasi, _ASI_CAMERA_INFO, _ASI_CONTROL_CAPS}, camera};

type ParamType = u32;
pub struct Camera {
    idx: i32,
    info : _ASI_CAMERA_INFO,
    ctl_params : HashMap<ParamType,_ASI_CONTROL_CAPS>

}

pub trait CameraControl{
    // main control
    fn open(&self,camera_id : i32);
    fn close(&self,camera_id : i32);
    fn init(&self, camera_id : i32);

    // parameter control
    // camera setting control

}

pub trait ParameterControl{

}
pub trait SettingControl{

}

impl Camera{
    pub fn new(camera_idx : i32) -> Self{
        // get number of camera
        let num_camera= libasi::get_num_of_connected_cameras();
        println!("num of camera devices {}", num_camera);

        // assigning connected camera_i property to camera_info
        let camera_info = libasi::get_camera_prop( camera_idx);
        println!("{:?}",camera_info);

        let mut camera = Camera{
                                    idx:camera_idx, 
                                    info : camera_info,
                                    ctl_params : HashMap::new()
        };

        let camera_id = camera_info.CameraID;
        camera.open(camera_id);
        camera.init(camera_id);

        let num_of_ctls = libasi::get_num_of_controls(camera_id );
        for ctl_idx in 0..num_of_ctls{
            let ctl_cpas = libasi::get_ctl_caps(camera_id, ctl_idx);
            camera.ctl_params.insert( ctl_cpas.ControlType, ctl_cpas );

        }
        println!("{:?}", camera.ctl_params);
        camera



    }
}
impl CameraControl for Camera{
    fn open(&self, camera_id : i32){
        libasi::open_camera(camera_id);
    }
    fn close(&self,camera_id : i32) {
        libasi::close_camera(camera_id);
        
    }
    fn init(&self, camera_id : i32) {
        libasi::init_camera(camera_id);
    }
}





mod test{
    use crate::libasi::_ASI_CAMERA_INFO;

    use super::*;

    #[test]
    fn test_get_prop(){
       

    }

}