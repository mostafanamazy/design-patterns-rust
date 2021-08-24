use crate::fly::{FlyBehavior, FlyNoWay, FlyWithWings};
use crate::quack::{Quack, QuackBehavior};
pub trait Displayer {
    fn display(&self);
}
pub struct DuckGeneric<F: FlyBehavior, Q: QuackBehavior, D: Displayer> {
    fly_behavior: F,
    quack_behavior: Q,
    displayer: D,
}
impl<F: FlyBehavior, Q: QuackBehavior, D: Displayer> DuckGeneric<F, Q, D> {
    pub fn new(fly_behavior: F, quack_behavior: Q, displayer: D) -> Self {
        DuckGeneric {
            fly_behavior,
            quack_behavior,
            displayer,
        }
    }
    pub fn perform_fly(&self) {
        self.fly_behavior.fly();
    }
    pub fn perform_quack(&self) {
        self.quack_behavior.quack();
    }
    pub fn swim(&self) {
        println!("All ducks float");
    }
    pub fn set_fly_behavior<U: FlyBehavior>(self, fly_behavior: U) -> DuckGeneric<U, Q, D> {
        DuckGeneric {
            fly_behavior,
            quack_behavior: self.quack_behavior,
            displayer: self.displayer,
        }
    }
    pub fn set_quack_behavior<N: QuackBehavior>(self, quack_behavior: N) -> DuckGeneric<F, N, D> {
        DuckGeneric {
            fly_behavior: self.fly_behavior,
            quack_behavior,
            displayer: self.displayer,
        }
    }
    pub fn display(&self) {
        self.displayer.display();
    }
}

pub struct MalarDuck {}
impl MalarDuck {
    pub fn new() -> DuckGeneric<FlyWithWings, Quack, MalarDuck> {
        DuckGeneric::new(FlyWithWings {}, Quack {}, MalarDuck {})
    }
}
impl Displayer for MalarDuck {
    fn display(&self) {
        println!("IM malar");
    }
}
pub struct ModDuck {}
impl ModDuck {
    pub fn new() -> DuckGeneric<FlyNoWay, Quack, ModDuck> {
        DuckGeneric::new(FlyNoWay {}, Quack {}, ModDuck {})
    }
}
impl Displayer for ModDuck {
    fn display(&self) {
        println!("IM Model");
    }
}
