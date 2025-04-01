pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    ((f - 32.0) * 5.0 / 9.0 * 1_000_000.0).round() / 1_000_000.0
}

pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    ((c * 9.0 / 5.0) + 32.0) * 1_000_000.0_f64.round() / 1_000_000.0_f64
}
