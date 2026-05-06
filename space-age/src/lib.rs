// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        todo!("convert a duration ({d:?}) to the number of years on this planet for that duration");
    }
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

const EARTH_SECONDS_PER_YEAR: f64 = 31_557_600.0;

impl Planet for Mercury {}
impl Planet for Venus {}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        d.seconds as f64 / EARTH_SECONDS_PER_YEAR
    }
}
impl Planet for Mars {}
impl Planet for Jupiter {}
impl Planet for Saturn {}
impl Planet for Uranus {}
impl Planet for Neptune {}
