const CARS_PER_HOUR_AND_SPEED_UNIT: u32 = 221;

pub fn production_rate_per_hour(speed: u8) -> f64 {
    let success_rate = match speed {
        0..=4 => 1.00,
        5..=8 => 0.90,
        9..=10 => 0.77,
        _ => panic!("Invalid speed: {}", speed),
    };

    let produced_cars = CARS_PER_HOUR_AND_SPEED_UNIT * u32::from(speed);

    f64::from(produced_cars) * success_rate
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
