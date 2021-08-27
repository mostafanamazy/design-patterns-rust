use crate::observer::Observer;
use std::cell::RefCell;
use std::rc::Rc;
pub trait Subject {
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>);
    fn notify_observers(&mut self);
}

pub struct WeatherData {
    observers: Vec<Rc<RefCell<dyn Observer>>>,
    temperature: f32,
    humidity: f32,
    pressure: f32,
}
impl WeatherData {
    pub fn new() -> WeatherData {
        WeatherData {
            observers: vec![],
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
    fn register_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.push(observer);
    }
    fn remove_observer(&mut self, observer: Rc<RefCell<dyn Observer>>) {
        self.observers.retain(|k| !Rc::ptr_eq(k, &observer));
    }
    fn notify_observers(&mut self) {
        for o in self.observers.iter() {
            o.borrow_mut()
                .update(self.temperature, self.humidity, self.pressure);
        }
    }
}
