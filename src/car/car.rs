pub enum EnigneType {
    ELECTRIC,
    GASOLINE,
    DIESEL,
}


pub struct Car {
    model: String,
    year: u32,
    speed: u32,
    engine_type: EnigneType,
}

impl Car {
    pub fn new(model: &str, year: u32, speed: u32, engine_type: EnigneType) -> Self {
        Car { 
            model: model.to_string(), 
            year: year,
            speed: speed,
            engine_type: engine_type,
        }
    }

    fn get_engine_type(&self) -> &str {
        match self.engine_type {
            EnigneType::ELECTRIC => "Electric",
            EnigneType::GASOLINE => "Gasoline",
            EnigneType::DIESEL => "Diesel",
        }
    }

    pub fn display_info(&self) {
        println!("Car model: {}, Year: {}, Speed: {}, Engine Type: {:?}", 
        self.model, self.year, self.speed, self.get_engine_type());
    }

    pub fn accelerate(&mut self, increment: u32) {
        self.speed += increment;
    }

}


