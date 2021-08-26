use crate::state::State;
use crate::noquarter::NoQuarterState;
use crate::soldout::SoldOutState;
use crate::gumball::GumballMachine;

pub struct SoldState{}
impl State for SoldState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("Please wait, we’re already giving you a gumball");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>{
        println!("Sorry, you already turned the crank");
        self
    }
    fn turn_crank(self: Box<Self>, _gumball: &GumballMachine) -> Box<dyn State>{
        println!("Turning twice doesn’t get you another gumball!");
        self
    }
    fn dispense(self: Box<Self>, gumball: &mut GumballMachine) -> Box<dyn State>{
        gumball.releas_ball();
        if gumball.count > 0 {
            Box::new(NoQuarterState{})
        } else {
            println!("Oops, out of gumballs!");
            Box::new(SoldOutState{})
        }
    }
}
