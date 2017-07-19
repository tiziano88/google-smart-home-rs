pub struct Light {
    pub id: String,
    pub name: String,
    pub status: LightStatus,
}

#[derive(Debug, Clone)]
pub struct LightStatus {
    pub on: bool,
    pub brightness: u64,
    pub spectrum_rgb: u64,
}

impl Default for LightStatus {
    fn default() -> LightStatus {
        LightStatus {
            on: false,
            brightness: 0,
            spectrum_rgb: 0,
        }
    }
}

impl Light {
    pub fn set_status(&mut self, s: &LightStatus) {
        println!("set status to: {:?}", s);
        self.status = s.clone();
    }

    pub fn get_status(&self) -> &LightStatus {
        &self.status
    }
}
