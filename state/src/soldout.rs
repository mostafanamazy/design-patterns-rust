use crate::state::State;
use crate::gumball::GumballMachine;

pub struct SoldOutState{}
impl State for SoldOutState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("We are soldOUt be cool!!!");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>{
        println!("We are soldOUt be cool!!!");
        self
    }
    fn turn_crank(self: Box<Self>, _gumball: &GumballMachine) -> Box<dyn State>{
        println!("We are soldOUt be cool!!!");
        self
    }
    fn dispense(self: Box<Self>, _gumball: &mut GumballMachine) -> Box<dyn State>{
        println!("We are soldOUt be cool!!!");
        self
    }
}
