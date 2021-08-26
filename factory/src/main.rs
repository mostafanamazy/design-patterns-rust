use factory::pizzastore::{ChicagoPizzaStore, NYPizzaStore};
fn main() {
    let ny_store = NYPizzaStore::new();
    let chicago_store = ChicagoPizzaStore::new();

    let pizza = ny_store.order_pizza("cheese");
    println!("Ethan ordered a {}", pizza.get_name());

    let pizza = chicago_store.order_pizza("cheese");
    println!("Joel ordered a {}", pizza.get_name());

    let pizza = chicago_store.order_pizza("veggie");
    println!("John ordered a {}", pizza.get_name());
}
