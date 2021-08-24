use crate::handler::Handler;

pub struct Handler3 {
    next: Option<Box<dyn Handler>>,
}
impl Handler3 {
    pub fn new() -> Handler3 {
        Handler3 { next: None }
    }
}
impl Handler for Handler3 {
    fn set_next(&mut self, n: Box<dyn Handler>) {
        self.next = Some(n);
    }
    fn handle(&self, i: u8) {
        if let Some(next) = &self.next {
            if i % 3 != 0 {
                // 3. Don't handle requests 3 times out of 4
                println!("H3 passed {}", i);
                next.handle(i); // 3. Delegate to the base class
                return;
            }
        }
        println!("H3 handled {}", i);
    }
}
