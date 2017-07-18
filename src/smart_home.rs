pub struct Light {
    pub id: String,
    pub name: String,
}

impl Light {
    pub fn SetStatus(&mut self, s: bool) {
        println!("set status to: {}", s);
    }
}
