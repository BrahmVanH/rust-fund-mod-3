// The HashMap<K, V> 
// Data type that stores data by mapping
// each key with its value
// Used for objects, hash tables, etc

use std::collections::HashMap;

// Provided code 
#[derive(PartialEq, Debug)]

struct Car { color: String, motor: Transmission, roof: bool, age: (Age, u32) }

#[derive(PartialEq, Debug)]
enum Transmission { Manual, SemiAuto, Automatic }

#[derive(PartialEq, Debug)]
enum Age { New, Used }



// Get the car quality by testing the value of the input argument
// - miles (u32)
// Return tuple with car age ("New" or "Used") and mileage
fn car_quality (miles: u32) -> (Age, u32) {

    // Check if car has accumulated miles
    // Return tuple early for Used car
    if miles > 0 {
        return (Age::Used, miles);
    }

    // Return tuple for New car, no need for "return" keyword or semicolon
    (Age::New, miles)
}

// Build "Car" using input arguments
fn car_factory(order: i32, miles: u32) -> Car {
    let colors = ["Blue", "Green", "Red", "Silver"];

    // Prevent panic: Check color index for colors array, reset as needed
    // Valid color = 1, 2, 3, or 4
    // If color > 4, reduce color to valid index
    let mut color = order as usize;
    if color > 4 {        
        // color = 5 --> index 1, 6 --> 2, 7 --> 3, 8 --> 4
        color = color - 4;
    }

    // Add variety to orders for motor type and roof type
    let mut motor = Transmission::Manual;
    let mut roof = true;
    if order % 3 == 0 {          // 3, 6, 9
        motor = Transmission::Automatic;
    } else if order % 2 == 0 {   // 2, 4, 8, 10
        motor = Transmission::SemiAuto;
        roof = false;
    }                            // 1, 5, 7, 11

    // Return requested "Car"
    Car {
        color: String::from(colors[(color-1) as usize]),
        motor: motor,
        roof: roof,
        age: car_quality(miles)
    }
}

fn main() {
    // Initialize hashmap to hold onto order details
    let mut orders: HashMap<i32, Car> = HashMap::new(); 

    // Initialize counter variable
    let mut order = 1;
    // Declare a car as mutable "Car" struct
    let mut car: Car;

    // Order 6 cars, increment "order" for each request
    // Car order #1: Used, Hard top
    car = car_factory(order, 1000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
    add_current_to_orders(&mut orders, car, order);
    
    for (key, value) in &orders {
      println!("butts {}: {:?}", key, value);
    } 
    // Car order #2: Used, Convertible
    order = order + 1;
    car = car_factory(order, 2000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);    
  add_current_to_orders(&mut orders, car, order);
    
    for (key, value) in &orders {
      println!("butts {}: {:?}", key, value);
    } 
    // Car order #3: New, Hard top
    order = order + 1;
    car = car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
  add_current_to_orders(&mut orders, car, order);
    
    for (key, value) in &orders {
      println!("butts {}: {:?}", key, value);
    } 
    // Car order #4: New, Convertible
    order = order + 1;
    car = car_factory(order, 0);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
  add_current_to_orders(&mut orders, car, order);
    
    for (key, value) in &orders {
      println!("butts {}: {:?}", key, value);
    } 
    // Car order #5: Used, Hard top
    order = order + 1;
    car = car_factory(order, 3000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
  add_current_to_orders(&mut orders, car, order);
    
    for (key, value) in &orders {
      println!("butts {}: {:?}", key, value);
    } 
    // Car order #6: Used, Hard top
    order = order + 1;
    car = car_factory(order, 4000);
    println!("{}: {:?}, Hard top = {}, {:?}, {}, {} miles", order, car.age.0, car.roof, car.motor, car.color, car.age.1);
}

fn add_current_to_orders(orders: &mut HashMap<i32, Car>, current_order: Car, order_number: i32) {
  orders.insert(order_number, current_order);
}

#[cfg(test)]
mod tests {
  use super::*;
  // #[test]
  // fn test_add_current_to_orders() {
  //   let mut orders: HashMap<i32, Car> = HashMap::new();
  //   let current_order: Car = Car::from(Car {color: String::from("Blue"), motor: Transmission::Manual, roof: true, age: (Age::Used, 12)});
  //   let order_number = 1;
  //   let mut result = orders
  //   result.insert(order_number, current_order);
  //   add_current_to_orders(&mut orders, current_order, order_number); 
  //   assert_eq!(orders, result); 
  // }
}
