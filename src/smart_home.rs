pub struct Light {
    pub id: String,
    pub name: String,
}

#[derive(Debug)]
pub struct LightStatus {
    pub on: bool,
    pub brightness: u64,
    pub spectrum_rgb: u64,
}

impl Light {
    pub fn set_status(&mut self, s: LightStatus) {
        println!("set status to: {:?}", s);
    }

    pub fn get_status(&mut self) -> LightStatus {
        LightStatus {
            on: true,
            brightness: 123,
            spectrum_rgb: 111,
        }
    }
}
