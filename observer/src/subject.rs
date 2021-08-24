use crate::observer::Observer;
use std::collections::HashMap;
pub trait Subject {
    fn register_observer(&mut self, key: &'static str, observer: Box<dyn Observer>);
    fn remove_observer(&mut self, key: &'static str);
    fn notify_observers(&mut self);
}

pub struct WeatherData {
    observers: HashMap<&'static str, Box<dyn Observer>>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
}
impl WeatherData {
    pub fn new() -> WeatherData {
        WeatherData {
            observers: HashMap::new(),
            temperature: 0.0,
            humidity: 0.0,
            pressure: 0.0,
        }
    }
}
impl WeatherData {
    pub fn set_measurements(&mut self, temperature: f32, humidity: f32, pressure: f32) {
        self.temperature = temperature;
        self.humidity = humidity;
        self.pressure = pressure;
        self.notify_observers();
    }
}
impl Subject for WeatherData {
    fn register_observer(&mut self, key: &'static str, observer: Box<dyn Observer>){
        self.observers.insert(key, observer);
    }
    fn remove_observer(&mut self, key: &'static str) {
        self.observers.remove(key);
    }
    fn notify_observers(&mut self) {
        for o in self.observers.values_mut() {
            o.update(self.temperature, self.humidity, self.pressure);
        }
    }
}
