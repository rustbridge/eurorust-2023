use thiserror::Error;

#[derive(Debug, Error)]
enum DogError {
    #[error("incorrect food amount (expected {expected:?}, given {given:?})")]
    IncorrectFoodAmount { expected: u32, given: u32 },
}

struct Dog {
    name: String,
    age: u32,
    size: DogSize,
}

impl Dog {
    pub fn new(name: &str, age: u32, size: DogSize) -> Self {
        Self {
            name: name.to_string(),
            age,
            size,
        }
    }
    pub fn say_hi(&self) -> String {
        format!(
            "Hi! My name is {} and I'm {} years old. Woof! Please feed me {} scoops",
            self.name,
            self.age,
            self.size.food_portion()
        )
    }

    pub fn feed(&self, amount: u32) -> Result<(), DogError> {
        Ok(self.size.check_portion(amount)?)
    }
}

pub enum DogSize {
    Large,
    Small,
}

impl DogSize {
    pub fn food_portion(&self) -> u32 {
        match self {
            DogSize::Large => 2,
            DogSize::Small => 1,
        }
    }

    fn check_portion(&self, amount: u32) -> Result<(), DogError> {
        let expected = self.food_portion();
        if amount == expected {
            Ok(())
        } else {
            Err(DogError::IncorrectFoodAmount { expected, given: amount })
        }
    }
}

fn main() {
    let mateo = Dog::new("Mateo", 4, DogSize::Large);
    let lu = Dog::new("Lu", 1, DogSize::Small);
    let dogs = [&mateo, &lu];

    for dog in dogs {
        let message = dog.say_hi();
        println!("{message}");
        match dog.feed(1) {
            Ok(_) => println!("A fed dog is a happy dog."),
            Err(e) => eprintln!("{}", e.to_string()),
        }
    }
}
