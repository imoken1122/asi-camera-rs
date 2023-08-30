mod camera;
mod libasi;
use camera::*;
fn main() {

        let asi_camera = ASIDevices::new();
        let camera =  asi_camera.get_camera(0);
        for ctl in camera.ctlcaps_mapper.iter() {
                println!("{:?} : {:?}", ctl.0, ctl.1.DefaultValue);
        }
        camera.set_img_type(libasi::ASI_IMG_TYPE_ASI_IMG_RAW8);
        println!("{:?}", camera.get_img_type());

        camera.close();
}
