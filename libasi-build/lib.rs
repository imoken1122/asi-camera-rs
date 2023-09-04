use log::*;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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
    pub fn display_info(&self) {
        let a : Vec<u8>= self.Name.iter().map(|&x| x as u8).collect();
        info!("===== Camera Info ======\n");
        info!("  Name: {:?}", String::from_utf8_lossy(&a));
        info!("  CameraID: {}", self.CameraID);
        info!("  MaxHeight: {}", self.MaxHeight);
        info!("  MaxWidth: {}", self.MaxWidth);
        info!("  IsColorCam: {}", self.IsColorCam);
        info!("  BayerPattern: {}", self.BayerPattern);
        info!("  SupportedBins: {:?}", &self.SupportedBins[..15]);
        info!("  SupportedVideoFormat: {:?}", &self.SupportedVideoFormat[..7]);
        info!("  PixelSize: {}", self.PixelSize);
        info!("  MechanicalShutter: {}", self.MechanicalShutter);
        info!("  ST4Port: {}", self.ST4Port);
        info!("  IsCoolerCam: {}", self.IsCoolerCam);
        info!("  IsUSB3Host: {}", self.IsUSB3Host);
        info!("  IsUSB3Camera: {}", self.IsUSB3Camera);
        info!("  ElecPerADU: {}", self.ElecPerADU);
        info!("  BitDepth: {}", self.BitDepth);
        info!("  IsTriggerCam: {}", self.IsTriggerCam);
        info!("======================\n");
    }
}



impl _ASI_SUPPORTED_MODE {
    pub fn new( ) -> Self{
        Self{SupportedCameraMode :[0;16]}
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