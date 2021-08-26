use crate::gumball::GumballMachine;
pub trait State {
    fn insert_quarter(self: Box<Self>) -> Box<dyn State>;
    fn eject_quarter(self: Box<Self>) -> Box<dyn State>;
    fn turn_crank(self: Box<Self>, gumball: &GumballMachine) -> Box<dyn State>;
    fn dispense(self: Box<Self>, gumball: &mut GumballMachine) -> Box<dyn State>;
}
