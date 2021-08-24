pub trait Handler {
    fn handle(&self, i: u8);
    fn set_next(&mut self, n: Box<dyn Handler>);
}
