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
  // Print a preparing message
  if miles > 0 {
    if roof == true {
      println!(
        "Preparing a used car: {:?}, {}, Hard top, {} miles",
        motor, color, miles
      );
    } else {
      println!(
        "Preparing a used car: {:?}, {}, Convertible, {} miles",
        motor, color, miles
      );
    }
  } else {
    println!(
      "Building a new car: {:?}, {}, Hard top, {} miles",
      motor, color, miles
    );
  }

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
  if miles > 0 {
    (Age::Used, miles)
  } else {
    (Age::New, 0)
  }
}

fn main() {
  // Car order #1: New, Manual, Hard top
  car_factory(String::from("Orange"), Transmission::Manual, true, 0);

  // Car order #2: Used, Semi-automatic, Convertible
  car_factory(String::from("Red"), Transmission::SemiAuto, false, 565);

  // Car order #3: Used, Automatic, Hard top
  car_factory(String::from("White"), Transmission::Automatic, true, 3000);
}
