use std::print;





struct Car {
    model: String,
    year: u32,
    speed: u32,
}

impl Car {
    fn new(model: &str, year: u32, speed: u32) -> Self {
        Car { 
            model: model.to_string(), 
            year: year,
            speed: speed
        }
    }

    fn display_info(&self) {
        println!("Car model: {}, Year: {}, Speed: {}", self.model, self.year, self.speed);
    }

    fn accelerate(&mut self, increment: u32) {
        self.speed += increment;
    }

}

fn main () {

    let mut my_car: Car = Car::new("Kia", 2016, 0);

    my_car.accelerate(50);

    my_car.display_info(); 

}
