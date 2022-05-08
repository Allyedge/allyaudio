pub struct AudioSettings {
    pub volume: f32,
    pub speed: f32,
}

impl AudioSettings {
    pub fn new(volume: f32, speed: f32) -> AudioSettings {
        AudioSettings { volume, speed }
    }
}
