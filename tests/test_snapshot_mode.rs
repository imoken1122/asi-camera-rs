
#[cfg(test)]
mod test{

    use asi_camera_rs::camera::*;
    use asi_camera_rs::libasi;
    #[test]
    fn test_snapshot_mode(){
        env_logger::init();
        let mut asi_camera = ASIDevices::new();
        let camera =  asi_camera.get_camera(0).read().unwrap();

        for ctl in camera.ctype2caps.iter() {
                println!("{:?} : {:?}", ctl.0, ctl.1.DefaultValue);
        }

        camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD ,
                         camera.ctype2caps.get(&libasi::ASI_CONTROL_TYPE_ASI_BANDWIDTHOVERLOAD).unwrap().MinValue, 0);
        camera.disable_dark_subtract();
        println!("{:?}", camera.get_img_type());
        camera.set_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE , 30, 0);
        println!("{:?}", camera.get_ctl_value(libasi::ASI_CONTROL_TYPE_ASI_EXPOSURE ));
        camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RGB24);
        println!("{:?}", camera.get_img_type());
        camera.snapshot();
        camera.close();


    }
   
}