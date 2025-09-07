struct Car {
    make: String,
    model: String,
    year: u32
}

impl Car {
    fn new(make: String, model: String, year: u32) -> Car {
        Car {
            make,
            model,
            year
        }
    }

    fn describe_car(&self) {
        println!("This is a {} {} {}", self.year, self.make, self.model)
    }
}



fn main() {
    let car_struct = Car::new(String::from("Ford"), String::from("Mustang"), 2022);
    car_struct.describe_car()
}

