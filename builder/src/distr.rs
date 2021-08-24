pub struct DistrWorkPackage {
    desc: String,
    temp: String,
}
impl DistrWorkPackage {
    pub fn new(mtype: &str) -> Self {
        DistrWorkPackage {
            desc: format!("Distributed Work Package for: {}", mtype),
            temp: "".into(),
        }
    }
    pub fn set_file(&mut self, f: &str, v: &str) {
        self.temp = format!("\n  File({}): {}", f, v);
        self.desc = format!("{}{}", self.desc, self.temp);
    }
    pub fn set_queue(&mut self, f: &str, v: &str) {
        self.temp = format!("\n  Queue({}): {}", f, v);
        self.desc = format!("{}{}", self.desc, self.temp);
    }
    pub fn set_pathway(&mut self, f: &str, v: &str) {
        self.temp = format!("\n  Pathway({}): {}", f, v);
        self.desc = format!("{}{}", self.desc, self.temp);
    }
    pub fn get_state(&self) -> &String {
        &self.desc
    }
}
