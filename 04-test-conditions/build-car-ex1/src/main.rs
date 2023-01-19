
/// Represents the transmission mechanism of a car.
#[derive(PartialEq, Debug)]
enum Transmission {
  Manual,
  SemiAuto,
  Automatic,
}

/// Represents the car quality.
#[derive(PartialEq, Debug)]
enum Age {
  /// A new car.
  New,
  /// An used car.
  Used,
}

/// Represents a car.
#[derive(PartialEq, Debug)]
struct Car {
  /// Color of the car.
  color: String,
  /// Transmission mechanism of the car.
  motor: Transmission,
  /// Whether the car has a roof.
  roof: bool,
  /// Mileage of the car
  age: (Age, u32),
}

/// Builds a `[Car]` by using values from the arguments.
///
/// # Arguments
///
/// * `color`: Color of car.
/// * `motor`: Mode of tansmission.
/// * `roof`: True if car has a hard top roof.
/// * `miles`: Mileage of the car.
fn car_factory(
  color: String,
  motor: Transmission,
  roof: bool,
  miles: u32,
) -> Car {
  Car {
    color: color,
    motor,
    roof,
    age: car_quality(miles),
  } // end fn car_factory()
}

/// Determines the age quality of the car.
///
/// # Arguments
///
/// * `miles`: Mileage of the car
fn car_quality(miles: u32) -> (Age, u32) {
  (Age::New, 0)
}

fn main() {
  let colors: [String; 4] = [
    "Blue".to_string(),
    "Green".to_string(),
    "Red".to_string(),
    "Silver".to_string(),
  ];

  let mut engine: Transmission = Transmission::Manual;

  let mut car: Car = Car {
    color: colors[2].clone(),
    motor: engine,
    roof: true,
    age: (Age::New, 0),
  };

  // Order 3 cars, one car for each type of transmission

  // Car order #1: New, Manual, Hard top
  engine = Transmission::Manual;
  car = car_factory(String::from(&colors[0]), engine, true, 0);
  println!(
    "Car order 1: {:?}, Hard top = {}, {:?}, {}, {} miles",
    car.age.0, car.roof, car.motor, car.color, car.age.1
  );

  // Car order #2: Used, Semi-automatic, Convertible
  engine = Transmission::SemiAuto;
  car = car_factory(String::from(&colors[1]), engine, false, 100);
  println!(
    "Car order 2: {:?}, Hard top = {}, {:?}, {}, {} miles",
    car.age.0, car.roof, car.motor, car.color, car.age.1
  );

  // Car order #3: Used, Automatic, Hard top
  engine = Transmission::Automatic;
  car = car_factory(String::from(&colors[2]), engine, true, 200);
  println!(
    "Car order 3: {:?}, Hard top = {}, {:?}, {}, {} miles",
    car.age.0, car.roof, car.motor, car.color, car.age.1
  );
}
