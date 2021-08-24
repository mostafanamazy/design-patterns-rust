use chain_of_responsibility::handler::Handler;
use chain_of_responsibility::handler1::Handler1;
use chain_of_responsibility::handler2::Handler2;
use chain_of_responsibility::handler3::Handler3;
fn main() {
    let root = Handler1::new();
    let mut two = Handler2::new();
    two.set_next(Box::new(root));
    let mut thr = Handler3::new();
    thr.set_next(Box::new(two));
    for i in 1..10 {
        thr.handle(i);
    }
}
