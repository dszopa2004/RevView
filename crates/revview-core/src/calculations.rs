// Convert kilometers per hour to miles per hour
pub fn kph_to_mph(kph: u16) -> u16 {
    (kph as f64 / 1.609344).round() as u16
}
