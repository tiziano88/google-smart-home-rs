use std::process::Command;

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
            brightness: 100,
            spectrum_rgb: 0xFFFFFF,
        }
    }
}

impl Light {
    pub fn set_on(&mut self, s: bool) -> &LightStatus {
        println!("set on to: {:?}", s);
        self.status.on = s;
        self.output();
        &self.status
    }

    pub fn set_brightness(&mut self, s: u64) -> &LightStatus {
        println!("set brightness to: {:?}", s);
        self.status.brightness = s;
        self.status.on = true;
        self.output();
        &self.status
    }

    pub fn set_spectrum_rgb(&mut self, s: u64) -> &LightStatus {
        println!("set spectrum_rgb to: {:?}", s);
        self.status.spectrum_rgb = s;
        self.status.on = true;
        self.output();
        &self.status
    }

    fn output(&self) {
        let r = (self.status.spectrum_rgb & 0xFF0000) >> 16;
        let g = (self.status.spectrum_rgb & 0x00FF00) >> 8;
        let b = (self.status.spectrum_rgb & 0x0000FF) >> 0;
        let scale = if self.status.on {
            self.status.brightness
        } else {
            0
        };
        let scaled_r = r * scale / 100;
        let scaled_g = g * scale / 100;
        let scaled_b = b * scale / 100;
        println!("scaled_r: {:?}", scaled_r);
        println!("scaled_g: {:?}", scaled_g);
        println!("scaled_b: {:?}", scaled_b);
        Command::new("python")
            .args(&["/home/pi/Pimoroni/mote/examples/rgb.py",
                    &format!("{}", scaled_r),
                    &format!("{}", scaled_g),
                    &format!("{}", scaled_b)])
            .output()
            .unwrap();
    }
}
