use crate::state::State;
use crate::noquarter::NoQuarterState;
use crate::sold::SoldState;
use crate::gumball::GumballMachine;

pub struct HasQuarterState{}
impl State for HasQuarterState {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State> {
        println!("You can’t insert another quarter");
        self
    }
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>{
        println!("Quarter returned");
        Box::new(NoQuarterState{})
    }
    fn turn_crank(self: Box<Self>, _gumball: &GumballMachine) -> Box<dyn State>{
        println!("You turned...”");
        Box::new(SoldState{})
    }
    fn dispense(self: Box<Self>, _gumball: &mut GumballMachine) -> Box<dyn State>{
        println!("No gumball dispensed");
        self
    }
}
