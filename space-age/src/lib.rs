#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(seconds: u64) -> Self {
        Self { seconds }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / (Self::period_in_earth_years() * EARTH_YEAR_IN_SECONDS)
    }
    fn period_in_earth_years() -> f64;
}

const EARTH_YEAR_IN_SECONDS: f64 = 31_557_600.0;

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_Planet {
    ($t:ty, $v:literal) => {
        impl Planet for $t {
            fn period_in_earth_years() -> f64 {
                $v
            }
        }
    };
}

impl_Planet!(Mercury, 0.240_846_7);
impl_Planet!(Venus, 0.615_197_26);
impl_Planet!(Earth, 1.0);
impl_Planet!(Mars, 1.880_815_8);
impl_Planet!(Jupiter, 11.862_615);
impl_Planet!(Saturn, 29.447_498);
impl_Planet!(Uranus, 84.016_846);
impl_Planet!(Neptune, 164.791_32);
