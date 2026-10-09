#[derive(Debug)]
pub struct Duration(u64);

impl Duration {
    fn as_secs_f64(&self) -> f64 {
        self.0 as f64
    }
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

const EARTH_YEAR_SECONDS: f64 = 31_557_600.0;

pub trait Planet {
    const ORBITAL_PERIOD: f64;

    fn years_during(d: &Duration) -> f64 {
        d.as_secs_f64() / (EARTH_YEAR_SECONDS * Self::ORBITAL_PERIOD)
    }
}

macro_rules! planet {
    ($name:ident, $period:expr) => {
        pub struct $name;

        impl Planet for $name {
            const ORBITAL_PERIOD: f64 = $period;
        }
    };
}

planet!(Mercury, 0.240_846_7);
planet!(Venus, 0.615_197_26);
planet!(Earth, 1.0);
planet!(Mars, 1.880_815_8);
planet!(Jupiter, 11.862_615);
planet!(Saturn, 29.447_498);
planet!(Uranus, 84.016_846);
planet!(Neptune, 164.791_32);
