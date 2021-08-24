pub trait Observer {
    fn update(&mut self, temp: f32, humidity: f32, pressure: f32);
}
