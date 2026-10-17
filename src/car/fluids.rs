use super::car::EngineType;

pub struct Fluids {
    pub oil: Option<f32>,
    pub coolant: f32,
    pub brake_fluid: f32,
}


impl Fluids {
    pub fn for_engine(engine_type: &EngineType) -> Self {
        match engine_type {
            EngineType::Electric => Fluids {
                oil: None,
                coolant: 0.0,
                brake_fluid: 0.0,
            },
            EngineType::Gasoline | EngineType::Diesel => Fluids {
                oil: Some(5.0),
                coolant: 3.0,
                brake_fluid: 1.0,
            },
        }
    }

    pub fn display_fluids(&self) {
        match self.oil {
            Some(oil_amount) => println!("Oil: {} liters, Coolant: {} liters, Brake Fluid: {} liters", oil_amount, self.coolant, self.brake_fluid),
            None => println!("No oil required. Coolant: {} liters, Brake Fluid: {} liters", self.coolant, self.brake_fluid),
        }
    }
}