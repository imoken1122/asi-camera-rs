<div align=center>
    <h1>asi-camera-rs</h1>
</div>

 This repository is a simplified Rust binding of Zwo ASI Camera driver.
 
The ASI Camera SDK is distributed [here](https://www.zwoastro.com/downloads/developers).
(This driver does not yet support MacOS Apple silicon.)


Tested OS
- MacOS Apple silicon M2 ( Rosseta2 ) 
- Ubuntu 22.04 x86_64

Other OS (Linux Windows, RasPi) and architecture  have not been tested because I don't have native environment.

## Run

First connect the ASI Camera using USB

Linux

```zsh
 RUST_LOG=debug cargo run -r --example snapshot_mode

 RUST_LOG=debug cargo run -r --example video_mode
```


MacOS

For MacOS, add  `--target x86_64-apple-darwin` 

```zsh
 
 rustup target add x86_64-apple-darwin

 RUST_LOG=debug cargo run -r --example snapshot_mode  --target x86_64-apple-darwin


```


## API call sequence


Official recommended order of function calls

An example implementations [snapshot_mode.rs](./examples/snapshot_mode.rs) and [video_mode.rs](./examples/video_mode.rs) is implemented according to the following order.

__1. Initialize__

1. ASIGetNumOfConnectedCameras: Get the count of connected cameras.
2. ASIGetCameraProperty: Retrieve camera information (ID, name, resolution, etc.).
3. ASIOpenCamera: Open a camera.
4. ASIInitCamera: Initialize the camera.
5. ASIGetNumOfControls: Get the count of control types.
6. ASIGetControlCaps: Retrieve the capacity of each control type.
7. ASISetROIFormat: Set image size and format.
8. ASISetStartPos: Set the starting position for the Region of Interest (ROI).


__2. Get and Set Control Value__

1. ASIGetControlValue: Get control values.
1. ASISetControlValue: Set control values (except for setting exposure time in trigger mode).

__3. Capture image__

- video mode
- snapshot mode