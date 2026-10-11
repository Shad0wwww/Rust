mod car;

use car::car::{ Car, EngineType};


fn main () {

    let mut my_car: Car = Car::new(
        "Kia", 
        2016, 
        0, 
        EngineType::Electric
    );

    my_car.accelerate(50);

    my_car.display_info(); 

}
