use crate::observer::Observer;
pub trait DisplayElement {
    fn display(&self);
}
pub struct CurrentConditionDisplay {
    temperature: f32,
    humidity: f32,
}
impl CurrentConditionDisplay {
    pub fn new() -> CurrentConditionDisplay {
        CurrentConditionDisplay {
            temperature: 0.0,
            humidity: 0.0,
        }
    }
}
impl Observer for CurrentConditionDisplay {
    fn update(&mut self, temp: f32, humidity: f32, _pressure: f32) {
        self.temperature = temp;
        self.humidity = humidity;
        self.display();
    }
}
impl DisplayElement for CurrentConditionDisplay {
    fn display(&self) {
        println!(
            "Current conditions: {} F degrees and {}% humidity",
            self.temperature, self.humidity
        );
    }
}

pub struct StatisticsDisplay {
    max: f32,
    min: f32,
    avg: f32,
    count: usize,
}
impl StatisticsDisplay {
    pub fn new() -> StatisticsDisplay {
        StatisticsDisplay {
            max: 0.0,
            min: 0.0,
            avg: 0.0,
            count: 0,
        }
    }
}
impl Observer for StatisticsDisplay {
    fn update(&mut self, temperature: f32, _humidity: f32, _pressure: f32) {
        if self.count == 0 {
            self.max = temperature;
            self.min = temperature;
            self.avg = temperature;
        } else {
            if temperature > self.max {
                self.max = temperature;
            } else if temperature < self.min {
                self.min = temperature;
            }
            self.avg = (self.count as f32 * self.avg + temperature) / (self.count + 1) as f32;
        }
        self.count += 1;
        self.display();
    }
}
impl DisplayElement for StatisticsDisplay {
    fn display(&self) {
        println!(
            "Avg/Max/Min temperature = {}/{}/{}",
            self.avg, self.max, self.min
        );
    }
}
