//! SigmaOS Tools Monologue - Webcam & Audio recording engine with timeline trimming.

/// Video capture profile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VideoProfile {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
}

impl VideoProfile {
    pub fn new(width: u32, height: u32, fps: u32) -> Self {
        Self { width, height, fps }
    }

    pub fn is_preferred(&self) -> bool {
        self.fps >= 30 && self.width >= 1280
    }
}

/// Device configuration selector.
#[derive(Debug, Clone)]
pub struct DeviceSelector {
    camera_id: String,
    mic_id: String,
    profiles: Vec<VideoProfile>,
}

impl DeviceSelector {
    pub fn new(camera_id: &str, mic_id: &str) -> Self {
        Self {
            camera_id: camera_id.to_string(),
            mic_id: mic_id.to_string(),
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, profile: VideoProfile) {
        self.profiles.push(profile);
    }

    pub fn best_profile(&self) -> Option<VideoProfile> {
        let mut valid: Vec<_> = self
            .profiles
            .iter()
            .filter(|p| p.fps >= 15)
            .cloned()
            .collect();
        valid.sort_by(|a, b| {
            let res_a = a.width * a.height;
            let res_b = b.width * b.height;
            res_b.cmp(&res_a).then_with(|| b.fps.cmp(&a.fps))
        });
        valid.first().cloned()
    }

    pub fn camera_id(&self) -> &str {
        &self.camera_id
    }

    pub fn mic_id(&self) -> &str {
        &self.mic_id
    }
}

/// Range trim editor: marks start and end cut points.
#[derive(Debug, Clone, PartialEq)]
pub struct TrimRange {
    pub start_sec: f64,
    pub end_sec: f64,
}

impl TrimRange {
    pub fn new(start_sec: f64, end_sec: f64) -> Result<Self, &'static str> {
        if start_sec < 0.0 || end_sec <= start_sec {
            return Err("Invalid trim range");
        }
        Ok(Self { start_sec, end_sec })
    }

    pub fn duration(&self) -> f64 {
        self.end_sec - self.start_sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profile_selection_aaa() {
        // Arrange
        let mut selector = DeviceSelector::new("video0", "hw:1,0");
        selector.add_profile(VideoProfile::new(640, 480, 30));
        selector.add_profile(VideoProfile::new(1920, 1080, 30));
        selector.add_profile(VideoProfile::new(3840, 2160, 10)); // Slow <15 fps mode should be skipped

        // Act
        let best = selector.best_profile().expect("Profile should be chosen");

        // Assert
        assert_eq!(best.width, 1920);
        assert_eq!(best.height, 1080);
        assert_eq!(best.fps, 30);
        assert!(best.is_preferred());
    }

    #[test]
    fn trim_range_calculation_aaa() {
        // Arrange & Act
        let range = TrimRange::new(1.5, 7.5).unwrap();

        // Assert
        assert_eq!(range.duration(), 6.0);
        assert!(TrimRange::new(5.0, 2.0).is_err());
    }
}
