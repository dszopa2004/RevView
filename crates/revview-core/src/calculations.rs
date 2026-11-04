// Convert kilometers per hour to miles per hour
pub fn kph_to_mph(kph: u16) -> u16 {
    (kph as f64 / 1.609344).round() as u16
}

// Converts celsius to fahrenheit
pub fn celsius_to_fahrenheit(celsius: i16) -> i16 {
    (celsius * 9 + 160 + 2) / 5
}
