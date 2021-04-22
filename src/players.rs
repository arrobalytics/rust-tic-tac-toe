use crate::board::{CellValue};

pub struct Player<'b> {
    first_name: String,
    last_name: String,
    age: i8,
    pub tick: &'b CellValue,
}

impl<'b> Player<'b> {
    pub fn new(first_name: &'static str, last_name: &'static str, age: i8, tick: &'b CellValue) -> Player<'b> {
        Player {
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            age,
            tick,
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    pub fn get_tick(&self) -> char {
        match self.tick {
            CellValue::X => { 'X' }
            CellValue::O => { 'O' }
            _ => { ' ' }
        }
    }

    pub fn say_hello(&self) {
        println!("Hello, I'm {} and I'm {}. My tick is {}. Let's play!", self.full_name(), self.age, self.get_tick())
    }
}