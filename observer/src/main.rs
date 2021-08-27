use observer::display::{CurrentConditionDisplay, StatisticsDisplay};
use observer::subject::{Subject, WeatherData};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut weather = WeatherData::new();
    let cur = Rc::new(RefCell::new(CurrentConditionDisplay::new()));
    weather.register_observer(cur);
    let stat = Rc::new(RefCell::new(StatisticsDisplay::new()));
    weather.register_observer(stat.clone());
    weather.set_measurements(80.0, 54.0, 30.4);
    weather.set_measurements(82.0, 70.0, 30.4);
    weather.set_measurements(78.0, 50.0, 30.4);
    weather.remove_observer(stat);
    weather.set_measurements(78.0, 50.0, 30.4);
}
