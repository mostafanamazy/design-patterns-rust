use observer::display::{CurrentConditionDisplay, StatisticsDisplay};
use observer::subject::{Subject, WeatherData};

static CONDITION_DISPLAY: &str  = "COND_DISP";
static STAT: &str = "STAT";
fn main() {
    let mut weather = WeatherData::new();
    let cur = CurrentConditionDisplay::new();
    weather.register_observer(CONDITION_DISPLAY, Box::new(cur));
    let stat = StatisticsDisplay::new();
    weather.register_observer(STAT, Box::new(stat));
    weather.set_measurements(80.0, 54.0, 30.4);
    weather.set_measurements(82.0, 70.0, 30.4);
    weather.set_measurements(78.0, 50.0, 30.4);
    weather.remove_observer(STAT);
    weather.set_measurements(78.0, 50.0, 30.4);
}
