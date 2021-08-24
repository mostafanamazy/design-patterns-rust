use crate::builder::Builder;
use crate::distr::DistrWorkPackage;
pub struct VmsBuilder {
    result: DistrWorkPackage,
}
impl VmsBuilder {
    pub fn new() -> Self {
        VmsBuilder {
            result: DistrWorkPackage::new("Vms"),
        }
    }
}
impl Builder for VmsBuilder {
    fn configure_file(&mut self, name: &str) {
        self.result.set_file("ISAM", name);
    }
    fn configure_queue(&mut self, queue: &str) {
        self.result.set_queue("priority", queue);
    }
    fn configure_pathway(&mut self, mtype: &str) {
        self.result.set_pathway("LWP", mtype);
    }
    fn get_result(&self) -> &DistrWorkPackage {
        &self.result
    }
}
