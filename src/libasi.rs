
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
//#![allow(improper_ctypes)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//use log::error;
pub type ASICameraInfo = _ASI_CAMERA_INFO;
pub type ASIControlCaps = _ASI_CONTROL_CAPS;
/// Structure to hold information about capabilities of a camera control.
pub type ASIBayerPattern = ASI_BAYER_PATTERN ; 
pub type ASIBool= u32;
///data format of image, such as 8bit, 16bit ...
pub type ASIImgType =  ASI_IMG_TYPE;
///The ASI_CAMERA_MODE enum defines different modes for automatic capturing in an ASI camera.
///These modes determine how the camera starts capturing images based on specific triggers or conditions, making it versatile for applications like astrophotography, motion detection, scientific experiments, and surveillance.
pub type ASICameraMode=  ASI_CAMERA_MODE;

pub type ASIExposureStatus= ASI_EXPOSURE_STATUS;
pub type ASIControlType= u32;
pub type ASIId = _ASI_ID;
pub type ASIControlValue = i64;

fn check_error_code(code: i32) {
    match code {
        // Success
        0 => (),
        // No camera connected or index value out of boundary
        1 => panic!("ASI_ERROR_INVALID_INDEX"),
        2 => panic!("ASI_ERROR_INVALID_ID"),
        3 => panic!("ASI_ERROR_INVALID_CONTROL_TYPE"),
        // Camera didn't open
        4 => panic!("ASI_ERROR_CAMERA_CLOSED"),
        // Failed to find the camera, maybe the camera has been removed
        5 => panic!("ASI_ERROR_CAMERA_REMOVED"),
        // Cannot find the path of the file
        6 => panic!("ASI_ERROR_INVALID_PATH"),
        7 => panic!("ASI_ERROR_INVALID_FILEFORMAT"),
        // Wrong video format size
        8 => panic!("ASI_ERROR_INVALID_SIZE"),
        9 => panic!("ASI_ERROR_INVALID_IMGTYPE"), //unsupported image formate
        10 => panic!("ASI_ERROR_OUTOF_BOUNDARY"), //the startpos is out of boundary
        // Communication timeout
        11 => panic!("ASI_ERROR_TIMEOUT"),
        12 => panic!("ASI_ERROR_INVALID_SEQUENCE"), //stop capture first!
        13 => panic!("ASI_ERROR_BUFFER_TOO_SMALL"), //buffer size is not big enough
        14 => panic!("ASI_ERROR_VIDEO_MODE_ACTIVE"),
        15 => panic!("ASI_ERROR_EXPOSURE_IN_PROGRESS"),
        16 => panic!("ASI_ERROR_GENERAL_ERROR"), //general error, eg: value is out of valid range
        17 => panic!("ASI_ERROR_INVALID_MODE"),  //the current mode is wrong
        18 => panic!("ASI_ERROR_END"),
        e => panic!("unknown error {}", e),
    }
}

impl _ASI_CAMERA_INFO {
    pub fn new() -> Self{
        Self{
            Name: [0; 64],
            CameraID: 0,
            MaxHeight: 0,
            MaxWidth: 0,
            IsColorCam: 0,
            BayerPattern: 0,
            SupportedBins: [0; 16],
            SupportedVideoFormat: [0; 8],
            PixelSize: 0.0,
            MechanicalShutter: 0,
            ST4Port: 0,
            IsCoolerCam: 0,
            IsUSB3Host: 0,
            IsUSB3Camera: 0,
            ElecPerADU: 0.0,
            BitDepth: 0,
            IsTriggerCam: 0,
            Unused: [0; 16],
        }
    }
}

impl _ASI_CONTROL_CAPS{
    pub fn new()->Self{
        Self{
            Name : [0; 64],
            Description : [0; 128],
            MaxValue : 0,
            MinValue : 0,
            DefaultValue : 0,
            IsAutoSupported : 0,
            IsWritable : 0,
            ControlType : 0,
            Unused : [0; 32]

        }
    }
}

impl _ASI_SUPPORTED_MODE {
    pub fn new( ) -> Self{
        Self{SupportedCameraMode :[0;16]}
    }
}


/// This should be the first API to be called.
/// Get the number of connected ASI cameras.
///
/// # Returns:
/// The number of connected ASI cameras. 1 means 1 camera is connected.
///
pub fn _get_num_of_connected_cameras() -> i32{
    unsafe{ASIGetNumOfConnectedCameras()} 

}

/// Open the camera before performing any operations on it. This will not affect a camera that is currently capturing.
/// All APIs below need the camera to be opened first.
///
/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
///
/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_CAMERA_REMOVED`: Failed to find the camera, maybe the camera has been removed.
///
pub fn _open_camera ( camera_id : i32  ){
    check_error_code(unsafe{ASIOpenCamera(camera_id)})

}

/// Close the camera to free all the resources.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: It will return success even if the camera is already closed.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _close_camera(camera_id: i32) {
    check_error_code(unsafe { ASICloseCamera(camera_id) });
}

/// Initialize the camera after opening. This function may take some time and can affect a camera that is currently capturing.
///
/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
///
/// # Returns:
/// - `ASI_SUCCESS`: The operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _init_camera( camera_id : i32  ){
    check_error_code(unsafe{ASIInitCamera(camera_id)})
}


/// Get the number of controls available for this camera. The camera needs to be opened first.
///
/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `pi_number_of_controls` (`*mut int`): Pointer to an `int` to save the number of controls.
///
/// # Returns:
/// - `ASI_SUCCESS`: The operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_num_of_controls( camera_id : i32, num_of_ctls : &mut i32) {
    check_error_code(unsafe{ASIGetNumOfControls(camera_id,  num_of_ctls)});
}

/// Get control properties available for this camera. The camera needs to be opened first.
/// The user needs to allocate and maintain the buffer.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `control_index` (`int`): Index of the control (NOT control type).
/// - `p_control_caps` (`*mut ASI_CONTROL_CAPS`): Pointer to a structure containing the property of the control.
///   The user needs to allocate the buffer for this.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_ctl_caps(camera_id : i32, ctl_idx: i32, ctl_caps : &mut _ASI_CONTROL_CAPS ) {
    check_error_code(unsafe{ASIGetControlCaps(camera_id, ctl_idx, ctl_caps)});


}


/// Get information about connected ASI cameras.
///
/// # Parameters:
/// - `p_asi_camera_info` (`*mut ASI_CAMERA_INFO`): Pointer to a structure containing the camera's properties.
///   The user needs to allocate the buffer for this.
/// - `camera_index` (`int`): The index of the camera. 0 means the first connected camera, 1 means the second connected camera, and so on.
///
/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_INVALID_INDEX`: No camera connected, or index value is out of boundary.

pub fn _get_camera_prop( camera_idx : i32) -> _ASI_CAMERA_INFO{

    let mut camera_info = _ASI_CAMERA_INFO::new();

    check_error_code(unsafe{ASIGetCameraProperty(&mut camera_info, camera_idx)});
    camera_info
}


/// Get the current Region of Interest (ROI) area setting.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `pi_width` (`*mut int`): Pointer to the width of the ROI area.
/// - `pi_height` (`*mut int`): Pointer to the height of the ROI area.
/// - `pi_bin` (`*mut int`): Pointer to binning method. bin1=1, bin2=2.
/// - `p_img_type` (`*mut ASI_IMG_TYPE`): Pointer to the output format.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_roi_format(camera_id :i32,width:&mut i32,height : &mut i32,bin : &mut i32,img_type : &mut ASIImgType) {
    check_error_code(unsafe{ASIGetROIFormat(camera_id,  width, height,  bin, img_type)})


}

/// Set the Region of Interest (ROI) area before capturing.
/// Capture must be stopped before calling this function.
/// The width and height are the values after binning.
/// call this function to change ROI area to the origin(center).
/// For example, if you want to run at 640X480@BIN2, you need to set width to 640 and height to 480.
/// Note that ASI120's data size must be a multiple of 1024, meaning width*height%1024=0.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `i_width` (`int`): The width of the ROI area. Make sure i_width%8 == 0.
/// - `i_height` (`int`): The height of the ROI area. Make sure i_height%2 == 0. Additionally, for USB2.0 camera ASI120, ensure i_width*i_height%1024=0.
/// - `i_bin` (`int`): Binning method. bin1=1, bin2=2.
/// - `img_type` (`ASI_IMG_TYPE`): The desired output format.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_INVALID_SIZE`: Wrong video format size.
/// - `ASI_ERROR_INVALID_IMGTYPE`: Unsupported image format. Make sure i_width, i_height, and binning are set correctly.
///
pub fn _set_roi_format(camera_id : i32, width: i32, height : i32, bin : i32, img_type : ASIImgType){
    
    check_error_code(unsafe{ASISetROIFormat(camera_id, width,height, bin,img_type)})
}
pub fn _get_position_of_roi(camera_id : i32 , x : &mut i32, y : &mut i32){

    check_error_code(unsafe{ASIGetStartPos(camera_id, x, y)});

}

/// Set the camera mode. This function only needs to be called when the `IsTriggerCam` in the `CameraInfo` is true.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `camera_mode` (`ASI_CAMERA_MODE`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_SEQUENCE`: The camera is in capture now; you need to stop capture first.
/// - `ASI_ERROR_INVALID_MODE`: The mode is out of boundary, or this camera does not support this mode.
///
pub fn _set_camera_mode(camera_id : i32, mode : ASICameraMode){

    check_error_code(unsafe{ASISetCameraMode(camera_id, mode)})
}

/// Start video capture. After starting, you can get the data from the `ASIGetVideoData` API.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful; it will return success if already started.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_EXPOSURE_IN_PROGRESS`: Snap mode is working; you need to stop snap first.
///
pub fn _start_video_capture(camera_id : i32){
    check_error_code(unsafe{ASIStartVideoCapture(camera_id)})

}

/// Call this API as quickly as possible, otherwise frames will be discarded.
/// The recommended approach is to maintain a buffer loop and call this API in a loop.
/// Ensure that the buffer size is large enough to hold one image; otherwise, this API might crash.
/// bufSize Byte length:for RAW8 and Y8，bufSize >= image_width*image_height，for RAW16，
/// bufSize >= image_width*image_height *2，for RGB24，bufSiz >= image_width*image_height *3 suggested iWaitms value: exposure_time*2 + 500ms

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `p_buffer` (`*mut u8`): Caller needs to allocate the buffer and ensure that the size is big enough.
///   The size in bytes:
///   - 8-bit mono: width * height
///   - 16-bit mono: width * height * 2
///   - RGB24: width * height * 3.
/// - `i_wait_ms` (`int`): This API will block and wait `i_wait_ms` to get one image. The unit is milliseconds.
///   Use -1 to wait forever. This value is recommended to be set to exposure * 2 + 500 ms.
/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_TIMEOUT`: No image obtained and timeout.
///
pub fn _get_video_data(camera_id : i32, pbuf : *mut u8,buf_size : i64, wait_ms : i32 ) {

    check_error_code(unsafe{ASIGetVideoData(camera_id, pbuf, buf_size, wait_ms)});
}



/// Stop video capture.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful; it will return success if already stopped.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _stop_video_capture(camera_id : i32){
    check_error_code(unsafe{ASIStopVideoCapture(camera_id )})

}

/// Start camera exposure. The following 4 APIs are usually used when long exposure is required.
/// Start exposure, check the exposure status, then get the data.
///
/// # Parameters:
/// - `camera_id` (`i32`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `is_dark` (`ASIBool`): Indicates whether it's a dark frame if there is a mechanical shutter on the camera. Otherwise, it's useless.
///
/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_VIDEO_MODE_ACTIVE`: Video mode is working; you need to stop video capture first.
///
pub fn _start_exposure(camera_id : i32, is_dark : ASIBool ){
    check_error_code(unsafe{ASIStartExposure(camera_id ,is_dark as i32 )})

}


/// To cancel the long exposure which is on.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _stop_exposure(camera_id : i32){
    check_error_code(unsafe{ASIStopExposure(camera_id )})

}

/// Get the exposure status, works with `ASIStartExposure`.
/// You can read the data if `ASI_EXP_SUCCESS` is obtained, or you have to restart exposure again if `ASI_EXP_FAILED`.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `p_exp_status` (`*mut ASI_EXPOSURE_STATUS`): The exposure status.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_exposure_status(camera_id : i32, exp_status : &mut ASIExposureStatus){

    check_error_code(unsafe{ASIGetExpStatus(camera_id, exp_status )})
}



/// Get data after exposure.
/// Please make sure the buffer size is big enough to hold one image; otherwise, this API will crash.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `p_buffer` (`*mut u8`): Caller needs to allocate the buffer and ensure that the size is big enough.
///   The size in bytes:
///   - 8-bit mono: width * height
///   - 16-bit mono: width * height * 2
///   - RGB24: width * height * 3.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_TIMEOUT`: No image obtained and timeout.
///
pub fn _get_data_after_exp(camera_id : i32, buf:*mut u8, buf_size: i64){

    check_error_code(unsafe{ASIGetDataAfterExp(camera_id, buf, buf_size)})
}




pub fn _get_id(camera_id: i32, asi_id: *mut ASIId) {
    check_error_code(unsafe { ASIGetID(camera_id, asi_id) });
}

pub fn _set_id(camera_id: i32, asi_id: ASIId) {
    check_error_code(unsafe { ASISetID(camera_id, asi_id) });
}



/// Provide a path to a dark file and enable dark subtraction.
/// This is used when there are hot pixels or a need for long exposure.
/// It is recommended to create the dark file using the "dark subtract" function 
/// of the "video capture filter" on the DirectShow page.
/// The dark file's size should be the same as the camera's max width and height, 
/// and it should be in RGB8 raw format. It will be applied even if you change the ROI setting.
/// It only corrects hot pixels if the output isn't 16-bit.
///
/// The "Dark subtract" setting will be remembered in the registry, so it will be on the next time you open your app.

/// # Parameters:
/// - `camera_id` (`i32`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `bmp_path` (`&std::ffi::CStr`): The path to the BMP dark file.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_PATH`: Cannot find the path of the file.
/// - `ASI_ERROR_INVALID_FILEFORMAT`: The dark file's size should be the same as the camera's max width and height.
///
pub fn _enable_dark_subtract(camera_id : i32, bmp_path : *mut i8){
    
    check_error_code(unsafe { ASIEnableDarkSubtract(camera_id, bmp_path) });
}



/// Disable the dark subtract function.
/// It is recommended to call it at the start if you don't want to use it, 
/// because the dark subtract function is remembered on the Windows platform.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
///
pub fn _disable_dark_subtract(camera_id : i32, ){
    
    check_error_code(unsafe { ASIDisableDarkSubtract(camera_id ) });
}

/// Get the value and auto value of control property.
/// Note: The value of the temperature is the float value multiplied by 10 to convert it to a long type.
/// The control name for temperature is "Temperature", and long is the only type for control 
/// (except cooler's target temperature, which is an integer).

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `control_type` (`int`): This is obtained from control property using the `ASIGetControlCaps` API.
/// - `pl_value` (`*mut i64`): Pointer to the value where you want to save the value obtained from control.
/// - `pb_auto` (`*mut ASIBool`): Pointer to the `ASIBool` type for auto value.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_INVALID_CONTROL_TYPE`: Invalid control type.
///
pub fn _get_ctl_value(camera_id : i32, ctl_type : ASIControlType,value :&mut ASIControlValue, is_auto : ASIBool) {
    check_error_code(unsafe { ASIGetControlValue(camera_id, ctl_type as i32, value,  &mut (is_auto as i32) ) });
}


/// Set control property value and auto value.
/// It will return success and set the max value or min value if the value is beyond the boundary.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `control_type` (`int`): This is obtained from control property using the `ASIGetControlCaps` API.
/// - `l_value` (`i64`): The value to set to the control.
/// - `b_auto` (`ASIBool`): Set the control to auto.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
/// - `ASI_ERROR_INVALID_CONTROL_TYPE`: Invalid control type.
/// - `ASI_ERROR_GENERAL_ERROR`: General error, e.g., value is out of valid range, or operation to camera hardware failed.

pub fn _set_ctl_value(camera_id : i32, ctl_type : ASIControlType, value : i64, is_auto : ASIBool){

    check_error_code(unsafe { ASISetControlValue(camera_id, ctl_type as i32, value, is_auto as i32 ) });
}

/// Get the camera's current mode.
/// This function should only be called when the `IsTriggerCam` in the `CameraInfo` is true.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `mode` (`*mut ASI_CAMERA_MODE`): The pointer to the current camera mode.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_camera_mode(camera_id : i32, mode : &mut ASI_CAMERA_MODE ) {

    check_error_code(unsafe { ASIGetCameraMode(camera_id, mode )});
}



/// Get the camera's supported mode.
/// This function should only be called when the `IsTriggerCam` in the `CameraInfo` is true.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `supported_mode` (`*mut ASI_SUPPORTED_MODE`): The pointer to the camera's supported mode.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///
pub fn _get_supported_mode(camera_id : i32, sup_mode : &mut _ASI_SUPPORTED_MODE){
    check_error_code(unsafe { ASIGetCameraSupportMode(camera_id,sup_mode)});

}

/// Get the dropped frames.
/// Dropped frames can occur when USB traffic is high or the hard disk write speed is slow.
/// The dropped frames counter will reset to 0 after stopping capture.

/// # Parameters:
/// - `camera_id` (`int`): This is obtained from the camera property using the `ASIGetCameraProperty` API.
/// - `pi_drop_frames` (`*mut int`): Pointer to the dropped frames count.

/// # Returns:
/// - `ASI_SUCCESS`: Operation is successful.
/// - `ASI_ERROR_CAMERA_CLOSED`: The camera didn't open.
/// - `ASI_ERROR_INVALID_ID`: No camera of this ID is connected, or the ID value is out of boundary.
///

pub fn _get_droped_frame(camera_id : i32, droped_frame : &mut i32) {

    check_error_code(unsafe { ASIGetDroppedFrames(camera_id,droped_frame)});
}