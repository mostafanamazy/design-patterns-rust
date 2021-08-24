use strategy::duck_box::{Duck, MallardDuck, ModelDuck};
use strategy::duck_generic::{MalarDuck, ModDuck};
use strategy::fly::FlyRocketPowered;
fn main() {
    let mallard = MallardDuck::new();
    mallard.perform_fly();
    mallard.perform_quack();
    mallard.display();
    mallard.swim();

    let mut model = ModelDuck::new();
    model.perform_fly();
    model.display();
    model.set_fly_behavior(Box::new(FlyRocketPowered {}));
    model.perform_fly();

    let mallard_duck = MalarDuck::new();
    mallard_duck.perform_fly();
    mallard_duck.perform_quack();
    mallard_duck.display();
    mallard_duck.swim();

    let model_duck = ModDuck::new();
    model_duck.perform_fly();
    let model_duck = model_duck.set_fly_behavior(FlyRocketPowered {});
    model_duck.perform_fly();
    model_duck.perform_quack();
    model_duck.display();
    model_duck.swim();
}
