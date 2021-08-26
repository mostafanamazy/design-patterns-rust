pub trait Pizza {
    fn bake(&self) {
        println!("bake for 25 minutes at 350");
    }
    fn cut(&self) {
        println!("Cutting the pizza into diagonal slices");
    }
    fn boxing(&self) {
        println!("Place pizza in official PizzaStore box");
    }
    fn prepare(&self);
    fn get_name(&self) -> &String;
}

pub struct ChicagoStyleCheesePizza {
    name: String,
    toppings: Vec<String>,
}

impl ChicagoStyleCheesePizza {
    pub fn new() -> Self {
        Self {
            name: "Chicago Style Deep Dish Cheese Pizza".to_string(),
            toppings: vec!["Shredded Mozzarella Cheese".to_string()],
        }
    }
}

impl Pizza for ChicagoStyleCheesePizza {
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauce...");
        println!("Adding toppings: ");
        for top in self.toppings.iter() {
            println!("   {}", top);
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
    fn cut(&self) {
        println!("Cutting the pizza into square slices");
    }
}

pub struct ChicagoStyleVeggiePizza {
    name: String,
    toppings: Vec<String>,
}

impl ChicagoStyleVeggiePizza {
    pub fn new() -> Self {
        Self {
            name: "Chicago Style Veggie Pizza".to_string(),
            toppings: vec!["Some Cheese".to_string()],
        }
    }
}

impl Pizza for ChicagoStyleVeggiePizza {
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauce...");
        println!("Adding toppings: ");
        for top in self.toppings.iter() {
            println!("   {}", top);
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct NYStyleCheesePizza {
    name: String,
    toppings: Vec<String>,
}

impl NYStyleCheesePizza {
    pub fn new() -> Self {
        Self {
            name: "NY Style Sauce and Cheese Pizza".to_string(),
            toppings: vec!["Grated Reggiano Cheese".to_string()],
        }
    }
}

impl Pizza for NYStyleCheesePizza {
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauce...");
        println!("Adding toppings: ");
        for top in self.toppings.iter() {
            println!("   {}", top);
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

pub struct NYStyleVeggiePizza {
    name: String,
    toppings: Vec<String>,
}

impl NYStyleVeggiePizza {
    pub fn new() -> Self {
        Self {
            name: "NY Style Sauce and Veggie Pizza".to_string(),
            toppings: vec!["Grated Reggiano Veggie".to_string()],
        }
    }
}

impl Pizza for NYStyleVeggiePizza {
    fn prepare(&self) {
        println!("Preparing {}", self.name);
        println!("Tossing dough...");
        println!("Adding sauce...");
        println!("Adding toppings: ");
        for top in self.toppings.iter() {
            println!("   {}", top);
        }
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}
