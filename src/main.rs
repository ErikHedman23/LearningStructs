use std::u8;
//the elements of a struct are called fields

#[derive(Debug, Clone)]
#[warn(dead_code)]
struct Shuttle {
    name: String,
    crew_size: u8,
    propellant: f64,
}
// This is a struct method. You use the same fn keyword when defining the method as you would a
// function, but the method will always begin with reference to the struct itself, i.e.: (&self)
impl Shuttle {
    fn get_name(&self) -> &str {
        &self.name
    }

    fn add_fuel(&mut self, gallons: f64) {
        self.propellant += gallons;
    }

    // You can also define associated functions (different from methods, they do not have a
    // reference to the struct (&self)) inside the implementation.  These can be used like a
    // constructor to create a new instance of the struct. These are typcially called new():
    fn new(name: &str) -> Shuttle {
        Shuttle {
            name: String::from(name),
            crew_size: 7,
            propellant: 8.9,
        }
    }
}

struct Color(u8, u8, u8); //This color represents a red, green, blue tuple

struct Point(u8, u8, u8); //xyz coordinates

fn get_y(p: Point) -> u8 {
    return p.1;
}

fn main() {
    // Creating an instance of the Shuttle struct; i.e, Instantiating:

    let mut vehicle = Shuttle {
        name: String::from("Endeavor"),
        crew_size: 7,
        propellant: 835958.0,
    };
    // To access the fields in a struct, you use dot . notation
    println!("name is {}", vehicle.name);

    let vehicle_name = vehicle.get_name();
    println!("The name of the vehicle is {}", vehicle_name);

    println!("The current propellant is {}", vehicle.propellant);
    vehicle.add_fuel(889077.23);
    println!("The current propellant is now at {}", vehicle.propellant);

    vehicle.name = String::from("Atlantis");

    println!("The name of the shuttle is now called {:?}", vehicle);

    let vehicle2 = Shuttle {
        name: String::from("Discovery"),
        ..vehicle
    };

    //Using the .. syntax allows you to clone the field data from one instance to another, but, we
    //cannot use it unless we derive the Clone trait to the struct, as shown on line 4:
    let vehicle3 = Shuttle { ..vehicle2.clone() };

    println!("Vehicle is {:?}", vehicle);
    println!("Vehicle 2 is {:?}", vehicle2);
    println!("Vehicle 3 is also {:?}", vehicle3);

    let new_vehicle = Shuttle::new("Hello");
    println!("The new vehicle is named: {}", new_vehicle.name);

    let coord = Point(4, 5, 6);
    let y = get_y(coord);
    println!("y is {}", y);
}
