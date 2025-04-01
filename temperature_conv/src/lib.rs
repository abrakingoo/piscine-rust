pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    ((f - 32.0) * 5.0 / 9.0).round() / 100.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    ((c * 9.0 / 5.0) + 32.0).round() / 100.0
}

fn main() {
    let fahrenheit = 20.0;
    let celsius = fahrenheit_to_celsius(fahrenheit);
    println!("{}°F is {}°C", fahrenheit, celsius);

    let celsius = 20.0;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    println!("{}°C is {}°F", celsius, fahrenheit);
}
