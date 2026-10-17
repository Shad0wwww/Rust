
use super::fluids::Fluids;
use rand::{Rng, distributions::{Alphanumeric, Uniform}};

#[derive(Debug, Copy, Clone)]
pub enum EngineType {
    Electric,
    Gasoline,
    Diesel,
}


pub struct Car {
    model: String,
    year: u32,
    speed: u32,
    engine_type: EngineType,
    numberplate: Option<String>,
    fluids: Fluids,
}

impl Car {
    pub fn new(model: &str, year: u32, speed: u32, engine_type: EngineType) -> Self {
        Car { 
            model: model.to_string(), 
            year: year,
            speed: speed,
            engine_type: engine_type,
            numberplate: Self::generate_number_plate(),
            fluids: Fluids::for_engine(&engine_type),
        }
    }

    fn get_engine_type(&self) -> &str {
        match self.engine_type {
            EngineType::Electric => "Electric",
            EngineType::Gasoline => "Gasoline",
            EngineType::Diesel => "Diesel",
        }
    }

    pub fn display_info(&self) {
        println!("Car model: {}, Year: {}, Speed: {}, Engine Type: {:?}, numberplate {}", 
        self.model, self.year, self.speed, self.get_engine_type(),
        self.numberplate.as_deref().unwrap_or("No numberplate"));

        self.fluids.display_fluids();
    }

    pub fn accelerate(&mut self, increment: u32) {
        self.speed += increment;
    }

    fn generate_number_plate() -> Option<String> {
        let random_string: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(2)
            .map(char::from)
            .collect();

        let random_numbers: Vec<i32> = rand::thread_rng()
            .sample_iter(Uniform::new_inclusive(0, 9))
            .take(5)
            .collect();

   

        Some(
            format!("{}-{}", 
                random_string.to_uppercase(), 
                random_numbers.iter().map(|n| n.to_string()).collect::<String>()
            )
        )
    
    }

}


