use crate::handler::Handler;

pub struct Handler1 {
    next: Option<Box<dyn Handler>>,
}
impl Handler1 {
    pub fn new() -> Handler1 {
        Handler1 { next: None }
    }
}
impl Handler for Handler1 {
    fn set_next(&mut self, n: Box<dyn Handler>) {
        self.next = Some(n);
    }
    fn handle(&self, i: u8) {
        if let Some(next) = &self.next {
            if i % 3 != 1 {
                // 3. Don't handle requests 3 times out of 4
                println!("H1 passed {}", i);
                next.handle(i); // 3. Delegate to the base class
                return;
            }
        }
        println!("H1 handled {}", i);
    }
}
