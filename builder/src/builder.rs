use crate::distr::DistrWorkPackage;

pub trait Builder {
    fn configure_file(&mut self, s: &str);
    fn configure_queue(&mut self, s: &str);
    fn configure_pathway(&mut self, s: &str);
    fn get_result(&self) -> &DistrWorkPackage;
}
