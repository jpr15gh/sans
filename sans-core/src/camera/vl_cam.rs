//! Hardware specific implementation for V4L2 cameras
//!
//!

use super::{Camera, CameraConfig, CameraError, CameraType};
pub use rscam::Camera as CameraBackend;
use rscam::Config;
use std::{fs, io::Write};

pub struct VLCamera {
    backend: CameraBackend,
    meta: CameraType,
}

impl VLCamera {
    /// Bind a new camera with a path on the FS
    pub fn new(path: &str, meta: CameraType) -> Result<VLCamera, CameraError> {
        let mut backend = match CameraBackend::new(path) {
            Ok(c) => c,
            Err(e) => {
                return Err(CameraError::ReceiverNotFound(format!(
                    "Failed to allocate '{}': {:?}",
                    path, e
                )))
            }
        };

        backend
            .start(&Config {
                interval: (1, 1), // 30 fps.
                resolution: (3840, 2160),
                // resolution: (4224, 3156),
                format: b"MJPG",
                ..Default::default()
            }).unwrap();

        Ok(VLCamera { backend, meta })
    }
}

impl Camera for VLCamera {
    fn auto_config(&mut self, _cfg: Option<CameraConfig>) -> Result<(), CameraError> {
        unimplemented!()
    }

    fn capture_image(&self) -> Result<(), CameraError> {
        let frame = self.backend.capture().unwrap();
        fs::remove_file(&format!("static/img/frame-{}.jpg", self.meta));
        let mut file = fs::File::create(&format!("static/img/frame-{}.jpg", self.meta)).unwrap();
        file.write_all(&frame[..]).unwrap();
        Ok(())
    }

    fn capture_video(&self, _fps: u32, _time: u32) -> Result<(), CameraError> {
        unimplemented!()
    }

    fn start_liveview(&self, _fps: u32) -> Result<(), CameraError> {
        unimplemented!()
    }

    fn stop_liveview(&self) -> Result<(), CameraError> {
        unimplemented!()
    }

    fn get_liveview_chunk(&self) {
        unimplemented!()
    }
}
