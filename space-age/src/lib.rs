// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(f64);

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration((s as f64) / EARTH_YEAR)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        d.0
    }
}
//Earth: orbital period 365.25 Earth days, or 31557600 seconds
//Mercury: orbital period 0.2408467 Earth years
//Venus: orbital period 0.61519726 Earth years
//Mars: orbital period 1.8808158 Earth years
//1.0 Earth years
//Jupiter: orbital period 11.862615 Earth years
//Saturn: orbital period 29.447498 Earth years
//Uranus: orbital period 84.016846 Earth years
//Neptune: orbital period 164.79132 Earth years
pub struct Mercury;

pub struct Venus;

pub struct Earth;

pub struct Mars;

pub struct Jupiter;

pub struct Saturn;

pub struct Uranus;

pub struct Neptune;

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 0.2408467
    }
}
const EARTH_YEAR:f64 = 31557600.0;

impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 0.61519726
    }
}

impl Planet for Earth {}

impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 1.8808158
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 11.862615
    }
}

impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 29.447498
    }
}

impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 84.016846
    }
}

impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
         d.0 / 164.79132
    }
}
