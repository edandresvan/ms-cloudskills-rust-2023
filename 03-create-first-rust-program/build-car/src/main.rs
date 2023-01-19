/// Represents the transmission mechanism of a car.
#[derive(PartialEq, Debug)]
enum Transmission {
  Manual,
  SemiAuto,
  Automatic,
}

/// Represents a car.
struct Car {
  color: String,
  transmission: Transmission,
  convertible: bool,
  mileage: u32,
}

/// Builds a `[Car]` by using values from the arguments.
///
/// # Arguments
///
/// * `color`: Color of car.
/// * `Transmission`: Mode of tansmission.
/// * `Convertible`: True if car is a convertible.
fn car_factory(
  color: String,
  transmission: Transmission,
  convertible: bool,
) -> Car {
  Car {
    color: color.clone(),
    transmission: transmission,
    convertible: convertible,
    mileage: 0,
  }
}
fn main() {
  let mut car = car_factory(String::from("Red"), Transmission::Manual, false);
  println!(
    "Car 1 = {}, {:?} transmission, convertible: {}, mileage: {}",
    car.color, car.transmission, car.convertible, car.mileage
  );

  car = car_factory(String::from("Silver"), Transmission::Automatic, true);
  println!(
    "Car 2 = {}, {:?} transmission, convertible: {}, mileage: {}",
    car.color, car.transmission, car.convertible, car.mileage
  );

  car = car_factory(String::from("Yellow"), Transmission::SemiAuto, false);
  println!(
    "Car 3 = {}, {:?} transmission, convertible: {}, mileage: {}",
    car.color, car.transmission, car.convertible, car.mileage
  );
}
