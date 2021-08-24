use crate::fly::{FlyBehavior, FlyNoWay, FlyWithWings};
use crate::quack::{Quack, QuackBehavior};
pub trait Duck {
    fn perform_fly(&self);
    fn perform_quack(&self);
    fn swim(&self) {
        println!("All ducks float");
    }
    fn display(&self);
    fn set_fly_behavior(&mut self, fly_b: Box<dyn FlyBehavior>);
    fn set_quack_behavior(&mut self, quack_b: Box<dyn QuackBehavior>);
}

pub struct MallardDuck {
    fly_behavior: Box<dyn FlyBehavior>,
    quack_behavior: Box<dyn QuackBehavior>,
}
impl MallardDuck {
    pub fn new() -> impl Duck {
        MallardDuck {
            fly_behavior: Box::new(FlyWithWings {}),
            quack_behavior: Box::new(Quack {}),
        }
    }
}
impl Duck for MallardDuck {
    fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
    fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    fn display(&self) {
        println!("I'm real MallardDuck");
    }
    fn set_fly_behavior(&mut self, fly_b: Box<dyn FlyBehavior>) {
        self.fly_behavior = fly_b;
    }
    fn set_quack_behavior(&mut self, quack_b: Box<dyn QuackBehavior>) {
        self.quack_behavior = quack_b;
    }
}

pub struct ModelDuck {
    fly_behavior: Option<Box<dyn FlyBehavior>>,
    quack_behavior: Option<Box<dyn QuackBehavior>>,
}
impl ModelDuck {
    pub fn new() -> impl Duck {
        ModelDuck {
            fly_behavior: Some(Box::new(FlyNoWay {})),
            quack_behavior: Some(Box::new(Quack {})),
        }
    }
}
impl Duck for ModelDuck {
    fn perform_fly(&self) {
        if let Some(f) = self.fly_behavior.as_ref() {
            f.fly();
        }
    }
    fn perform_quack(&self) {
        if let Some(q) = self.quack_behavior.as_ref() {
            q.quack();
        }
    }
    fn display(&self) {
        println!("I'm real ModelDuck");
    }
    fn set_fly_behavior(&mut self, fly_b: Box<dyn FlyBehavior>) {
        self.fly_behavior = Some(fly_b);
    }
    fn set_quack_behavior(&mut self, quack_b: Box<dyn QuackBehavior>) {
        self.quack_behavior = Some(quack_b);
    }
}
