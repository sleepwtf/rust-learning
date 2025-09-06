struct Car {
    make: String,
    model: String,
    year: u32
}

fn describe_car(car_struct: &Car) {
    println!("This is a {} {} {}", car_struct.year, car_struct.make, car_struct.model)
}


fn main() {
    let car_struct = Car {
        make: String::from("Ford"), 
        model: String::from("Mustang"), 
        year: 2022
    };
    describe_car(&car_struct)
}

