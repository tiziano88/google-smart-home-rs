pub struct Scene {
    pub id: String,
    pub name: String,
    pub reversible: bool,
}

impl Scene {
    pub fn activate_scene(&mut self, deactivate: bool) {
        debug!("activate_scene: {:?}", deactivate);
    }
}
