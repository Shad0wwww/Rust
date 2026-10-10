mod car;

use car::car::Car;
use car::car::EnigneType;

fn main () {

    let mut my_car: Car = Car::new("Kia", 2016, 0, EnigneType::ELECTRIC);

    my_car.accelerate(50);

    my_car.display_info(); 

}
