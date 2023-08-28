

use crate::api::*;


pub struct Camera {
}



impl Camera {
    pub fn init(self){
        let num_device = get_num_of_connected_cameras();
        println!("num of camera devices {}", num_device);

    }

}





mod test{
    use super::*;

    #[test]
    fn test_camera(){
        let camera = Camera{};
        camera.init();
    }

}