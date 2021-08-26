use crate::noquarter::NoQuarterState;
use crate::soldout::SoldOutState;
use crate::state::State;
pub struct GumballMachine {
    state: Option<Box<dyn State>>,
    pub count: usize,
}
impl GumballMachine {
    pub fn new(count: usize) -> GumballMachine {
        GumballMachine {
            state: if count > 0 {
                Some(Box::new(NoQuarterState{}))
            }else {
                Some(Box::new(SoldOutState{}))
            },
            count,
        }
    }
    pub fn insert_quarter(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.insert_quarter());
        }
    }
    pub fn eject_quarter(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.eject_quarter());
        }
    }
    pub fn turn_crank(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.turn_crank(self).dispense(self));
        }
    }
    pub fn releas_ball(&mut self) {
        println!("A gumball comes rolling out the slot..");
        if self.count > 0 {
            self.count -=1;
        }
    }
}
