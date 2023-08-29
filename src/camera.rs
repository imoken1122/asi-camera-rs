
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{libasi::{self as libasi, _ASI_CAMERA_INFO, _ASI_CONTROL_CAPS, ROIFormat, ControlState, ASIBool, _get_supported_mode}, camera};

type ParamType = u32;

#[derive(Debug,Clone)]
pub struct ASICameras{
    pub cameras : Vec<Camera>
} 
impl ASICameras {
    pub fn new() -> Self{

        let num_camera= libasi::_get_num_of_connected_cameras();
        println!("num of camera devices {}", num_camera);
        let mut cameras = vec![];
        for i in 0..num_camera{
            let camera = Camera::new(i);
            cameras.push(camera);
        }
        Self{cameras}

    }
    pub fn get_camera(&self, camera_idx : i32) -> &Camera {
        if self.cameras.len()> camera_idx as usize {
            &self.cameras[camera_idx as usize]
        }
        else{
            panic!("Invaild index : index out bound");

        }

    }
}
#[derive(Debug,Clone)]
pub struct Camera {
    pub idx: i32,
    pub id: i32,
    pub info : _ASI_CAMERA_INFO,
    //pub ctl_state : HashMap<ParamType,ControlState>

}

pub trait CameraControl{
    // main control
    fn open(&self);
    fn close(&self);
    fn init(&self );

    // parameter control
    // camera setting control

}

pub trait ParameterControl{
    fn get_roi_format(&self )->libasi::ROIFormat;
    fn set_roi_format(&mut self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType);
    fn get_ctl_value(&self,  ctl_type : libasi::ASIControlType) -> libasi::ControlState;
    fn set_ctl_value(&mut self, ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto: libasi::ASIBool);
    fn get_mode(&self, )->libasi::ASICameraMode;
    fn get_supported_mode(&self,) -> libasi::_ASI_SUPPORTED_MODE;

}

impl Camera{
    pub fn new(camera_idx : i32) -> Self{
        // assigning connected camera_i property to camera_info
        let camera_info = libasi::_get_camera_prop( camera_idx);
        println!("{:?}",camera_info);

        let camera_id = camera_info.CameraID;
        let mut camera = Camera{
                                    id : camera_id,
                                    idx: camera_idx, 
                                    info : camera_info,
         //                           ctl_state : HashMap::new()
        };

        camera.open();
        camera.init();

        //let num_of_ctls = libasi::_get_num_of_controls(camera_id );
        //for ctl_idx in 0..num_of_ctls{
        //    let ctl_cpas = libasi::_get_ctl_caps(camera_id, ctl_idx);
         //   println!("{:?}", ctl_cpas );
          //  camera.ctl_state.insert( ctl_cpas.ControlType, ctl_cpas );

        // }
        let modes = _get_supported_mode(camera_id);
         println!("{:?}", modes);

        camera



    }
}
impl CameraControl for Camera{
    fn open(&self ){
        let camera_id = self.id;
        libasi::_open_camera(camera_id);
    }
    fn close(&self,) {
        let camera_id = self.id;
        libasi::_close_camera(camera_id);
        
    }
    fn init(&self) {
        let camera_id = self.id;
        libasi::_init_camera(camera_id);
    }
}

impl ParameterControl for Camera {
    fn get_roi_format(&self, ) -> libasi::ROIFormat{
        let camera_id = self.id;
        libasi::_get_roi_format(camera_id)

    }
    fn set_roi_format(&mut self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType){
        let camera_id = self.id;
        libasi::_set_roi_format(camera_id, width, height, bin, img_type);
    }
    fn get_ctl_value(&self, ctl_type : libasi::ASIControlType) -> libasi::ControlState{
        let camera_id = self.id;
        libasi::_get_ctl_value(camera_id, ctl_type)
        
    }
    fn set_ctl_value(&mut self,ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto : libasi::ASIBool) {
        let camera_id = self.id;
        libasi::_set_ctl_value(camera_id, ctl_type,value, is_auto);

    }
    fn get_mode(&self,)->libasi::ASICameraMode {
        let camera_id = self.id;
        libasi::_get_camera_mode(camera_id)
    }
    fn get_supported_mode(&self,) -> libasi::_ASI_SUPPORTED_MODE {
        let camera_id = self.id;
        libasi::_get_supported_mode(camera_id)
    }
}



mod test{
    use crate::libasi::_ASI_CAMERA_INFO;

    use super::*;

    #[test]
    fn test_get_prop(){
       

    }

}