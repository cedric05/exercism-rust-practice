// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug)]
pub struct Duration(u64);

impl Duration {
    fn time(&self) -> u64 {
        self.0
    }
}
impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration(s)
    }
}

pub trait Planet {
    fn years_during(d: &Duration) -> f64 {
        unimplemented!(
            "convert a duration ({:?}) to the number of years on this planet for that duration",
            d,
        );
    }
}

macro_rules! planet_impl {
    ($t:ident, $n:expr) => {
        pub struct $t {}
        impl Planet for $t {
            fn years_during(d: &Duration) -> f64 {
                (d.time() as f64 / 31556952.0) / $n
            }
        }
    };
}

planet_impl![Mercury, 0.2408467];
planet_impl![Venus, 0.61519726];
planet_impl![Earth, 1.0];
planet_impl![Mars, 1.8808158];
planet_impl![Jupiter, 11.862615];
planet_impl![Saturn, 29.447498];
planet_impl![Uranus, 84.016846];
planet_impl![Neptune, 164.79132];
