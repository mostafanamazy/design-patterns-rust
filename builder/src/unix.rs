use crate::builder::Builder;
use crate::distr::DistrWorkPackage;
pub struct UnixBuilder {
    result: DistrWorkPackage,
}
impl UnixBuilder {
    pub fn new() -> Self {
        UnixBuilder {
            result: DistrWorkPackage::new("Unix"),
        }
    }
}
impl Builder for UnixBuilder {
    fn configure_file(&mut self, name: &str) {
        self.result.set_file("flatFile", name);
    }
    fn configure_queue(&mut self, queue: &str) {
        self.result.set_queue("FIFO", queue);
    }
    fn configure_pathway(&mut self, mtype: &str) {
        self.result.set_pathway("thread", mtype);
    }
    fn get_result(&self) -> &DistrWorkPackage {
        &self.result
    }
}
