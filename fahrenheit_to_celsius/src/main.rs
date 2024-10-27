fn main() {
    let value_in_fahrenheit = 98.0;
    let value_in_celsius = convert_to_celsius(value_in_fahrenheit);

    println!("{value_in_fahrenheit}F is equal to {value_in_celsius}C")
}

fn convert_to_celsius(value: f32) -> f32 {
    (value - 32.0) * (5.0 / 9.0)
}
