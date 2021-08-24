use crate::handler::Handler;

pub struct Handler2 {
    next: Option<Box<dyn Handler>>,
}
impl Handler2 {
    pub fn new() -> Handler2 {
        Handler2 { next: None }
    }
}
impl Handler for Handler2 {
    fn set_next(&mut self, n: Box<dyn Handler>) {
        self.next = Some(n);
    }
    fn handle(&self, i: u8) {
        if let Some(next) = &self.next {
            if i % 3 != 2 {
                // 3. Don't handle requests 3 times out of 4
                println!("H2 passed {}", i);
                next.handle(i); // 3. Delegate to the base class
                return;
            }
        }
        println!("H2 handled {}", i);
    }
}
