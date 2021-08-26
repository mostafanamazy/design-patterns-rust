use crate::createpizza::{ChicagoStylePizzaStore, CreatePizza, NYStylePizzaStore};
use crate::pizza::Pizza;
pub struct PizzaStore<C: CreatePizza> {
    creator: C,
}
impl<C: CreatePizza> PizzaStore<C> {
    pub fn order_pizza(&self, ptype: &str) -> Box<dyn Pizza> {
        let pizza = self.creator.create_pizza(ptype);
        pizza.prepare();
        pizza.bake();
        pizza.cut();
        pizza.boxing();
        pizza
    }
}

pub struct NYPizzaStore {}
impl NYPizzaStore {
    pub fn new() -> PizzaStore<NYStylePizzaStore> {
        PizzaStore {
            creator: NYStylePizzaStore {},
        }
    }
}

pub struct ChicagoPizzaStore {}
impl ChicagoPizzaStore {
    pub fn new() -> PizzaStore<ChicagoStylePizzaStore> {
        PizzaStore {
            creator: ChicagoStylePizzaStore {},
        }
    }
}
