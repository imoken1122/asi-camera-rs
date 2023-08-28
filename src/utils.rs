
pub mod Init{
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
}