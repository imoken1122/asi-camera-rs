
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use crate::utils;
use image;
use image::DynamicImage;
use core::panic;
use std::collections::HashMap;
use crate::libasi;
use std::fs::File;
use std::io::Write;

pub type BufSize = i64;
pub type BufType = Vec<u8>;

#[derive(Debug, Copy, Clone)]
pub struct ROIFormat{
    pub camera_id : i32,
    pub width : i32,
    pub height : i32,
    pub bin : i32,
    pub img_type : libasi::ASIImgType

}
impl ROIFormat { 
    pub fn new() -> Self{
        ROIFormat { camera_id: 0, width: 0, height: 0, bin: 0, img_type: 0 }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct ControlState{
    pub value : libasi::ASIControlValue,
    pub is_auto : libasi::ASIBool,

} 
#[derive(Debug,Clone,)]
pub struct ASIDevices{
    pub devices: Vec<Camera>
}
impl ASIDevices {
    pub fn new() ->  Self{
        
        let num_camera= libasi::_get_num_of_connected_cameras();
        info!("Num of camera devices {}", num_camera);
        if num_camera == 0{
            panic!("Not exist to connected cameras");
        }
        let mut devices= vec![];
        for i in 0..num_camera{
            let camera = Camera::new(i);
            devices.push( camera);
        }
         Self{ devices } 

    }
    pub fn get_camera(&self, camera_idx : i32) -> Camera {
        match self.devices.len()> camera_idx as usize{
            true => self.devices[camera_idx as usize].clone(),
            false => panic!("Invaild index : index out bound {}",camera_idx)
        }

    }
    pub fn get_num_of_connected_cameras(&self) -> i32 {
        libasi::_get_num_of_connected_cameras()

    }
}
#[derive(Debug,Clone)]
pub struct Camera {
    pub idx: i32,
    pub id: i32,
    pub info : libasi::_ASI_CAMERA_INFO,
    pub ctype2caps: HashMap<libasi::ASIControlType, libasi::ASI_CONTROL_CAPS>,

}

pub trait CameraControl{
    // main control
    fn open(&self);
    fn close(&self);
    fn init(&self );
    fn start_video_capture(&self);
    fn stop_video_capture(&self);
    fn start_exposure(&self, is_dark : libasi::ASIBool); 
    fn stop_exposure(&self); 
    fn get_exposure_status(&self) -> libasi::ASIExposureStatus;
    fn disable_dark_subtract(&self);

    fn get_num_of_controls(&self,) -> i32;
    fn get_ctl_caps_by_idx(&self,ctl_idx:i32) -> libasi::_ASI_CONTROL_CAPS;
    fn get_ctl_caps(&self, ctl_type:libasi::ASIControlType) -> libasi::_ASI_CONTROL_CAPS;
    fn get_roi_format(&self )->ROIFormat;
    fn set_roi_format(&self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType);
    fn get_ctl_value(&self,  ctl_type : libasi::ASIControlType) -> ControlState;
    fn set_ctl_value(&self, ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto: libasi::ASIBool);
    fn get_mode(&self, )->libasi::ASICameraMode;
    fn get_supported_mode(&self,) -> libasi::_ASI_SUPPORTED_MODE;
    fn get_position_of_roi(&self) -> Vec<i32>;

    fn set_img_type(&self,img_type : libasi::ASIImgType); 
    fn get_img_type(&self) -> libasi::ASIImgType;
    fn get_dropeed_frame(&self,)->i32;
    fn get_video_data(&self,pbuf : Option<*mut u8>, wait_ms:i32) -> Option<BufType>; 
    fn get_data_after_exposure(&self)->BufType;

}

pub trait CameraService{
    fn snapshot(&self );
    fn capture_video_frame(&self, auto_adjust_ctls: Option<Vec<libasi::ASIControlType>>);
    fn auto_adjust_ctl_value(&self,ctl_type: libasi::ASIControlType) ;
}
pub trait ImageProcessor{
    fn save_img(&self,dyn_img: DynamicImage , extention : &str);
    fn save_buffer(&self, buf : BufType, extention : &str );
    fn create_buffer(&self, buf_size:BufSize) -> BufType;
    fn buf_to_img(&self,buffer:BufType, img_type : libasi::ASIImgType) -> DynamicImage;
    fn get_buffer_size(&self,) -> BufSize;
    fn buf_to_fits(&self, buf : BufType) -> BufType;
}

impl Camera{
    pub fn new(camera_idx : i32) -> Self{
        // assigning connected camera_i property to camera_info
        let camera_info = libasi::_get_camera_prop( camera_idx);
        camera_info.display_info();

        let camera_id = camera_info.CameraID;
        let mut camera = Camera{
                                    id : camera_id,
                                    idx: camera_idx, 
                                    info : camera_info,
                                    ctype2caps: HashMap::new(),
                                    
        };

        camera.open();
        camera.init();

        let num_of_ctls = camera.get_num_of_controls();
        info!("Num of control types {}",num_of_ctls);
        for ctl_idx in 0..num_of_ctls{
            let ctl_cpas =camera.get_ctl_caps_by_idx(ctl_idx);
            camera.ctype2caps.insert( ctl_cpas.ControlType, ctl_cpas);
        }

        camera



    }
}
impl CameraControl for Camera{
    fn open(&self ){
        libasi::_open_camera(self.id);
    }
    fn close(&self,) {
        libasi::_close_camera(self.id);
        
    }
    fn init(&self) {
        libasi::_init_camera(self.id);
    }
    fn start_exposure(&self, is_dark : libasi::ASIBool){
        libasi::_start_exposure(self.id, is_dark);
    }
    fn get_exposure_status(&self,) -> libasi::ASIExposureStatus{
        let mut exp_status : libasi::ASIExposureStatus = 0;
        libasi::_get_exposure_status(self.id, &mut exp_status);
        exp_status
    }
    fn stop_exposure(&self){
        libasi::_stop_exposure(self.id);
    }
    fn start_video_capture(&self ) {
        libasi::_start_video_capture(self.id);

    }
    fn stop_video_capture(&self ) {
        libasi::_stop_video_capture(self.id);
    }
    fn disable_dark_subtract(&self) {
        libasi::_disable_dark_subtract(self.id);
    }
    fn get_roi_format(&self ) -> ROIFormat{
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
    fn get_ctl_caps_by_idx(&self,ctl_idx:i32) -> libasi::_ASI_CONTROL_CAPS {
        
        let mut ctl_caps = libasi::_ASI_CONTROL_CAPS::new();
        libasi::_get_ctl_caps(self.id, ctl_idx, &mut ctl_caps);
        ctl_caps
    }

    fn get_ctl_caps(&self,ctl_type:libasi::ASIControlType) -> libasi::_ASI_CONTROL_CAPS {
        *self.ctype2caps.get(&ctl_type).unwrap()
    }
    fn get_num_of_controls(&self,) ->i32{

        let mut num_of_ctls : i32 = 0 ; 
        libasi::_get_num_of_controls(self.id, &mut num_of_ctls);
        num_of_ctls

    }
    fn set_roi_format(&self,  width : i32, height:i32, bin : i32, img_type : libasi::ASIImgType){
        libasi::_set_roi_format(self.id, width, height, bin, img_type);
    }
    fn get_ctl_value(&self, ctl_type : libasi::ASIControlType) -> ControlState{
        let mut value : libasi::ASIControlValue = 0;
        let mut is_auto : libasi::ASIBool = 0;
        libasi::_get_ctl_value(self.id, ctl_type, &mut value, is_auto);
        ControlState{value, is_auto}
        
    }
    fn set_ctl_value(&self,ctl_type : libasi::ASIControlType, value : libasi::ASIControlValue, is_auto : libasi::ASIBool) {
        libasi::_set_ctl_value(self.id, ctl_type,value, is_auto);

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
    fn set_img_type(&self,img_type : libasi::ASIImgType) {
        let roi = self.get_roi_format();
        libasi::_set_roi_format(self.id, roi.width, roi.height, roi.bin, img_type);
    }
    fn get_img_type(&self) -> libasi::ASIImgType {
        let roi = self.get_roi_format();
        roi.img_type
    }
   
    fn get_dropeed_frame(&self,)->i32 {
        let mut droped_frame  : i32 = 0;
        libasi::_get_droped_frame(self.id, &mut droped_frame);
        droped_frame
    }


    /// When a pointer to buffer is received, the video frame is written with the pointer at the beginning
    /// and the return value is None. Conversely, if the buffern pointer is None, the video frame is returned.
    /// 
    fn get_video_data(&self,pbuf : Option<*mut u8> ,wait_ms : i32) -> Option<BufType >{

        let buf_size = self.get_buffer_size();
        match pbuf {
            Some(pbuf) => {
                libasi::_get_video_data(self.id,pbuf,buf_size,wait_ms  );
                None

            }
            None => {
                let mut buf = self.create_buffer(buf_size);
                let mut pbuf = buf.as_mut_ptr();
                libasi::_get_video_data(self.id,pbuf,buf_size,wait_ms  );
                Some(buf)

            }

        }
    }
    fn get_data_after_exposure(&self)->BufType {
        
        let buf_size = self.get_buffer_size();
        let mut buf = self.create_buffer(buf_size);
        let mut pbuf = buf.as_mut_ptr();
        libasi::_get_data_after_exp(self.id,pbuf,buf_size);
        buf
    }
}

impl CameraService for Camera{
    fn snapshot(&self ) { 
        // starting exposure
        info!("Starting exposure");
        self.start_exposure(0);

        // Loop until exposure time ends. (status is working)
        while self.get_exposure_status() == libasi::ASI_EXPOSURE_STATUS_ASI_EXP_WORKING {
            std::thread::sleep(std::time::Duration::from_millis(50));

        }

        let exp_status  = self.get_exposure_status();
        match exp_status { 
            libasi::ASI_EXPOSURE_STATUS_ASI_EXP_SUCCESS =>  info!("Successfly exposure"),
            other => panic!("Failed to snapshot. exposure status is {}",other) 
        }

        // Acquire data after exposure
        let buf = self.get_data_after_exposure();
       debug!("buffer size is {}", buf.len());

        // convert buffer to image format
       let img_type = self.get_img_type();
       match img_type { 
            // RAW16
            libasi::ASI_IMG_TYPE_ASI_IMG_RAW16 => {
                let fits = self.buf_to_fits(buf);
                // save buffer
                self.save_buffer(fits,"fit");
            },
            // RGB24,RAW8
            _ =>{
                let dyn_img = self.buf_to_img(buf, img_type);
                // save image
                self.save_img(dyn_img, "png");
            }
       }

        self.stop_exposure();
        info!("Stopped  exposure");

    }
    /// 
    fn capture_video_frame(&self,auto_adjust_ctls: Option<Vec<libasi::ASIControlType>>) {
        info!("Starting capture video frames");

       self.start_video_capture(); 

       let img_type = self.get_img_type();
       let mut n_capture=  0;

       // Num of capture
       while n_capture < 10 { 
        
            //std::thread::sleep(std::time::Duration::from_secs_f32(0.5));

            // whether control value auto adjust 
            if let Some(ctl_typs) = &auto_adjust_ctls {
                // auto adust by each control type
                for &ctl_typ in ctl_typs {
                    self.auto_adjust_ctl_value(ctl_typ);
                }
            } else {
                info!("Not setting auto adjust control value");
            }

            // recommended wait(ms) time in official docs
            let wait_ms : i32 = (self.get_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE).value/1000) as i32 * 2 + 500;
            let buf = self.get_video_data(None,wait_ms).unwrap();
            //self.save_buffer(data);
            let dyn_img = self.buf_to_img(buf, img_type);
            self.save_img(dyn_img, "png");


            let n_dropped = self.get_dropeed_frame();
            debug!("cumulative total of drop frame {}\n",n_dropped);

            n_capture += 1;

       }
        std::thread::sleep(std::time::Duration::from_secs_f32(1.0));
       self.stop_video_capture();
        info!("{} frame captured", n_capture);
        info!("Stopped capture video frame");

    }


   

    /// Auto adjust value of givien control typej
    fn auto_adjust_ctl_value(&self,ctl_type:libasi::ASIControlType) {
        // get control caps
        let caps = self.get_ctl_caps(ctl_type );

        // Whether this contorl type supports auto-adjustment.
        if libasi::ASI_BOOL_ASI_TRUE != caps.IsAutoSupported{
                error!("Control type  {} is not supported for automatic",ctl_type);
                return
        }
        // set is_auto to true
        self.set_ctl_value(ctl_type , caps.DefaultValue, libasi::ASI_BOOL_ASI_TRUE );

        let mut prev_v = self.get_ctl_value(ctl_type).value; 
        let mut prev_df = self.get_dropeed_frame();
        let mut n_match = 0;
        let threshold = 3;
        //Loop until control value converges
        while n_match < threshold{ 
            let v= self.get_ctl_value(ctl_type).value;
            let df = self.get_dropeed_frame();
            if df == prev_df { continue;}
            if prev_v - v == 0{
                n_match+=1;
            }
            prev_v = v;
            prev_df = df;

        }
        debug!("Control type {} value updated to {}",ctl_type,prev_v);

        
    }

}

impl ImageProcessor for Camera {
    fn save_img(&self,dyn_img : DynamicImage, extention : &str )  {
        // TODO 
        // supoorte FITS extentino
        let ext = match extention{
            "jpg"=> image::ImageFormat::Jpeg,
            "png"=> image::ImageFormat::Png,
            _ => panic!("Not supported image extension")
            };
        let output_path = utils::generate_filename(extention);
        match dyn_img.save_with_format(output_path.clone(), ext) {
            Ok(()) => info!("Image saved to {}", output_path),
            Err(e) => panic!("Failed to save image : {}",e)
        }
    }
    fn save_buffer(&self ,buf : BufType,extention : &str ) {
        let output_path = utils::generate_filename(extention);
        let mut file = match File::create(&output_path){
            Ok(file) => file,
            Err(e) => {
                eprintln!("Failed to create file : {:?}", e);
                return;
            }
        };
    
        // バッファの内容をファイルに書き込む
        match file.write_all(&buf) {
            Ok(_) => info!("Buffer saved to  {}",output_path),
            Err(e) => eprintln!("Failed to save buffer {:?}", e),
        }
    }
    fn buf_to_img(&self,buffer:BufType, img_type : libasi::ASIImgType) -> DynamicImage {
        let roi = self.get_roi_format();
        let width = roi.width as u32;
        let height= roi.height as u32;
     // convert to image by image type (RAW8,RAW16,RGB24,Y8)
        let dyn_img  = match img_type{
            libasi::ASI_IMG_TYPE_ASI_IMG_RGB24=> DynamicImage::ImageRgb8(
                                                    image::RgbImage::from_raw(
                                                            width, 
                                                            height,
                                                             buffer.to_vec()).unwrap()),
            libasi::ASI_IMG_TYPE_ASI_IMG_RAW8 => DynamicImage::ImageLuma8(
                                                    image::GrayImage::from_raw(width, height,buffer.to_vec()).unwrap()),
            

            _ => panic!("Not supoorted image type")

            };
        dyn_img

    }
    fn get_buffer_size(&self) -> BufSize{

        let roi = self.get_roi_format();
        let mut buf_size : i64 = roi.width as i64 * roi.height as i64;

        // IMG_RAW8 and IMG_Y8 is 1 byte, same size with w*h
        buf_size = match roi.img_type {
            libasi::ASI_IMG_TYPE_ASI_IMG_RAW16 => buf_size * 2,
            libasi::ASI_IMG_TYPE_ASI_IMG_RGB24 => buf_size * 3,
             _ => buf_size

            };
        buf_size

    }
    fn create_buffer(&self,buf_size: BufSize) -> BufType {
        vec![0; buf_size as usize]
    }

    /// buffer convert to fits format 
    fn buf_to_fits(&self,buf : BufType) -> BufType {
        // amount of padding
        fn padding(n:usize ) -> usize{
            match n % 2880 {
                0 => 0,
                a => 2880 - a
            }
        }

        let mut fits = Vec::new();
        let roi = self.get_roi_format();
        let img_t = roi.img_type;
        
        let (w, h) = (if roi.width < 1000 { format!(" {}", roi.width) } else { format!("{}", roi.width) }, 
                                    if roi.height < 1000 { format!(" {}", roi.height) } else { format!("{}", roi.height) });

        let bit = match img_t{
                            libasi::ASI_IMG_TYPE_ASI_IMG_RAW16 => "16",
                            libasi::ASI_IMG_TYPE_ASI_IMG_RAW8 => " 8",
                            _ => panic!("Fits format is not supported RGB format")
                        };


        // section number of row is 80
        let header =
                            [ "SIMPLE  =                    T / FITS standard                                  ",
                                &("BITPIX  =                   ".to_owned()+bit+" / bits per pixel                                 "),
                                "NAXIS   =                    2 / number of axis                                 ",
                                &("NAXIS1   =                ".to_owned()+w.as_str()+" / length of data axis 1                          "),
                                &("NAXIS2   =                ".to_owned()+h.as_str()+" / length of data axis 2                          "),
                              "END                                                                             "
                            ];

        // header section
         for (i,h) in header.into_iter().enumerate(){
            if h.len()!=80{
               error!("length of header {} is {}, its must be 80.",i,h.len());
            }
            for b in h.as_bytes(){
                fits.push(*b);
            }
         }

        // length of header section and data section is 2880
        // padding using empty(32) to 2880
         for _ in 0..padding(fits.len()){
            fits.push(32);
         }

         // data section 
         for data in &buf{
            fits.push(*data)
         }
        
         fits

    }
}

