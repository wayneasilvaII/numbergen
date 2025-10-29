use rand::prelude::*; // brings Rng trait and rng() into scope

#[derive(Debug)]
pub struct NumberGenerator {
    number: i32, //32 bit signed integer
}

impl NumberGenerator {
    pub fn new() -> Self {
        let mut rng = rand::rng(); // fast, pre-initialized generator
        let num: i32 = rng.random_range(1..=1000); // random number between 1 and 1000
        
        Self { number: num } // return the struct with the random number
    }

    pub fn number(&self) -> i32 {
        self.number
    }
}

// example usage in main()
/*
fn main() {
    let generator = NumberGenerator::new(); //create new instance of NumberGenerator struct

    println!("{:#?}", generator.number); //print the random number
}
*/

// test
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_number_in_range() {
        let g = NumberGenerator::new();
        assert!(g.number() >= 1 && g.number() <= 1000);
    }
}
