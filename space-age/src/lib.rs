#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

pub trait Planet {
    fn period_in_earth_years() -> f64;

    #[must_use]
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (Self::period_in_earth_years() * EARTH_YEAR_IN_SECONDS)
    }
}

macro_rules! define_planet {
    ($planet_name:ident, $period_length:literal) => {
        pub struct $planet_name;

        impl Planet for $planet_name {
            fn period_in_earth_years() -> f64 {
                $period_length
            }
        }
    };
}

define_planet!(Mercury, 0.240_846_7);
define_planet!(Venus, 0.615_197_26);
define_planet!(Earth, 1.0);
define_planet!(Mars, 1.880_815_8);
define_planet!(Jupiter, 11.862_615);
define_planet!(Saturn, 29.447_498);
define_planet!(Uranus, 84.016_846);
define_planet!(Neptune, 164.791_32);
