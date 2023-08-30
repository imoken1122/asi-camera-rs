
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::collections::HashMap;

use crate::{libasi::{self as libasi, _ASI_CAMERA_INFO, _ASI_CONTROL_CAPS, ROIFormat, ControlState, ASIBool, _get_supported_mode, ASI_CONTROL_TYPE}, camera};

type ParamType = u32;

#[derive(Debug,Clone)]
pub struct ASICameras{
    pub cameras : Vec<Camera>
} 
impl ASICameras {
    pub fn new() -> Self{

        let num_camera= libasi::_get_num_of_connected_cameras();
        println!("num of camera devices {}", num_camera);
        if num_camera == 0{
            panic!("no connected cameras");
        }
        let mut cameras = vec![];
        for i in 0..num_camera{
            let camera = Camera::new(i);
            cameras.push(camera);
        }
        Self{cameras}

    }
    pub fn get_camera(&self, camera_idx : i32) -> &Camera {
        if self.cameras.len()> camera_idx as usize{
            &self.cameras[camera_idx as usize]
        }
        else{
            panic!("Invaild index : index out bound {}",camera_idx);

        }

    }
}
#[derive(Debug,Clone)]
pub struct Camera {
    pub idx: i32,
    pub id: i32,
    pub info : _ASI_CAMERA_INFO,
    pub ctlcaps_mapper: HashMap<ASI_CONTROL_TYPE, _ASI_CONTROL_CAPS>

}

pub trait CameraControl{
    // main control
    fn open(&self);
    fn close(&self);
    fn init(&self );
    fn start_video_capture(&self);
    fn stop_video_capture(&self);
    fn get_video_data(&self,wait_ms:i32) -> Vec<u8>;
    fn get_num_of_connected_cameras(&self) -> i32;
    // parameter control
    // camera setting control

}

pub trait ParameterControl{
    fn get_num_of_controls(&self,) -> i32;
    fn get_ctl_caps(&self,ctl_idx:i32) -> libasi::_ASI_CONTROL_CAPS;
    fn get_roi_format(&self )->libasi::ROIFormat;
    fn set_roi_format(&mut self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType);
    fn get_ctl_value(&self,  ctl_type : libasi::ASIControlType) -> libasi::ControlState;
    fn set_ctl_value(&mut self, ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto: libasi::ASIBool);
    fn get_mode(&self, )->libasi::ASICameraMode;
    fn get_supported_mode(&self,) -> libasi::_ASI_SUPPORTED_MODE;
    fn get_position_of_roi(&self) -> Vec<i32>;

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
                                    ctlcaps_mapper: HashMap::new()
        };

        camera.open();
        camera.init();

        let num_of_ctls = camera.get_num_of_controls();
        for ctl_idx in 0..num_of_ctls{
            let ctl_cpas =camera.get_ctl_caps(ctl_idx);
            println!("{:?}", ctl_cpas );
            camera.ctlcaps_mapper.insert( ctl_cpas.ControlType, ctl_cpas);
        }

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
    fn get_num_of_connected_cameras(&self) -> i32 {
        libasi::_get_num_of_connected_cameras()

    }
    fn start_video_capture(&self ) {
        libasi::_start_video_capture(self.id);

    }
    fn stop_video_capture(&self ) {
        libasi::_stop_video_capture(self.id);


    }
    fn get_video_data(&self, wait_ms : i32) -> Vec<u8> {
        let roi = self.get_roi_format();
        let mut buf_size : i64 = roi.width as i64 * roi.height as i64;
        let img_type = roi.img_type;
        let mut buf = Vec::new();

        // 2bytes per pixel
        if img_type == libasi::ASI_IMG_TYPE_ASI_IMG_RAW16 {
            buf_size *= 2;
        } 
        // 3bytes per pixel
        else if img_type == libasi::ASI_IMG_TYPE_ASI_IMG_RGB24{
            buf_size *= 3;
        }
        buf.resize(buf_size as usize, 0);
        let mut pbuf = buf.as_mut_ptr();
        libasi::_get_video_data(self.id,pbuf,buf_size,wait_ms  );
        buf
    }
}

impl ParameterControl for Camera {
    fn get_roi_format(&self ) -> libasi::ROIFormat{
        let camera_id = self.id;

        let mut width : i32 = 0;
        let mut height : i32= 0;
        let mut bin :i32 = 1;
        let mut img_type:i32= 0;
        libasi::_get_roi_format(camera_id,&mut width,&mut height,&mut bin,&mut img_type);
        ROIFormat {
            camera_id,
            width,
            height,
            bin,
            img_type
        }

    }
    fn get_ctl_caps(&self,ctl_idx:i32) -> libasi::_ASI_CONTROL_CAPS {
        
        let mut ctl_caps = libasi::_ASI_CONTROL_CAPS::new();
        libasi::_get_ctl_caps(self.id, ctl_idx, &mut ctl_caps);
        ctl_caps
    }
    fn get_num_of_controls(&self,) ->i32{

        let mut num_of_ctls : i32 = 0 ; 
        libasi::_get_num_of_controls(self.id, &mut num_of_ctls);
        num_of_ctls

    }
    fn set_roi_format(&mut self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType){
        let camera_id = self.id;
        libasi::_set_roi_format(camera_id, width, height, bin, img_type);
    }
    fn get_ctl_value(&self, ctl_type : libasi::ASIControlType) -> libasi::ControlState{
        let mut value : libasi::ASIControlValue = 0;
        let mut is_auto : libasi::ASIBool = 0;
        libasi::_get_ctl_value(self.id, ctl_type, &mut value, &mut is_auto);
        libasi::ControlState{value, is_auto}
        
    }
    fn set_ctl_value(&mut self,ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto : libasi::ASIBool) {
        let camera_id = self.id;
        libasi::_set_ctl_value(camera_id, ctl_type,value, is_auto);

    }
    fn get_mode(&self,)->libasi::ASICameraMode {
        let mut mode : libasi::ASICameraMode= 0;
        libasi::_get_camera_mode(self.id, &mut mode);
        mode
    }
    fn get_supported_mode(&self,) -> libasi::_ASI_SUPPORTED_MODE {
        let mut sup_mode = libasi::_ASI_SUPPORTED_MODE::new(); 
        libasi::_get_supported_mode(self.id,&mut sup_mode);
        sup_mode
    }
    fn get_position_of_roi(&self) -> Vec<i32>{
        let mut x =0;
        let mut y =0;
        libasi::_get_position_of_roi(self.id, &mut x, &mut y);
        vec![x,y]
    }

}



mod test{
    use crate::libasi::_ASI_CAMERA_INFO;

    use super::*;

    #[test]
    fn test_get_prop(){
       

    }

}