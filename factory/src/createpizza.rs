use crate::pizza::{
    ChicagoStyleCheesePizza, ChicagoStyleVeggiePizza, NYStyleCheesePizza, NYStyleVeggiePizza, Pizza,
};
pub trait CreatePizza {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza>;
}

pub struct NYStylePizzaStore {}

impl CreatePizza for NYStylePizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        if pizza_type == "veggie" {
            Box::new(NYStyleVeggiePizza::new())
        } else {
            Box::new(NYStyleCheesePizza::new())
        }
    }
}
pub struct ChicagoStylePizzaStore {}

impl CreatePizza for ChicagoStylePizzaStore {
    fn create_pizza(&self, pizza_type: &str) -> Box<dyn Pizza> {
        if pizza_type == "veggie" {
            Box::new(ChicagoStyleVeggiePizza::new())
        } else {
            Box::new(ChicagoStyleCheesePizza::new())
        }
    }
}
