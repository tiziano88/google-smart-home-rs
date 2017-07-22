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
            brightness: 0,
            spectrum_rgb: 0,
        }
    }
}

impl Light {
    pub fn set_on(&mut self, s: bool) -> &LightStatus {
        println!("set on to: {:?}", s);
        if s {
            Command::new("python")
                .args(&["/home/pi/Pimoroni/mote/examples/rgb.py", "255", "255", "255"])
                .output()
                .unwrap();
        } else {
            Command::new("python")
                .args(&["/home/pi/Pimoroni/mote/examples/rgb.py", "0", "0", "0"])
                .output()
                .unwrap();
        }
        self.status.on = s;
        &self.status
    }

    pub fn set_brightness(&mut self, s: u64) -> &LightStatus {
        println!("set brightness to: {:?}", s);
        Command::new("python")
            .args(&["/home/pi/Pimoroni/mote/examples/rgb.py",
                    &format!("{}", s),
                    &format!("{}", s),
                    &format!("{}", s)])
            .output()
            .unwrap();
        self.status.brightness = s;
        &self.status
    }

    pub fn set_spectrum_rgb(&mut self, s: u64) -> &LightStatus {
        println!("set spectrum_rgb to: {:?}", s);
        self.status.spectrum_rgb = s;
        &self.status
    }
}
