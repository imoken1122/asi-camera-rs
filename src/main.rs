mod camera;
mod libasi;
use camera::*;
fn main() {

        let asi_camera = ASICameras::new();
        let camera_i =  asi_camera.get_camera(0);
        println!("{:?}", camera_i.get_mode());
        camera_i.close();
}
