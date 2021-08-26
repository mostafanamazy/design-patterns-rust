use state::gumball::GumballMachine;
fn main() {
    let mut gumball = GumballMachine::new(5);

    gumball.insert_quarter();
    gumball.turn_crank();

    gumball.insert_quarter();
    gumball.turn_crank();
    gumball.insert_quarter();
    gumball.eject_quarter();
}
