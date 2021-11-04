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

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

fn fun_name(d: &Duration, factor: f64) -> f64 {
    (d.time() as f64 / 31556952.0) / factor
}

impl Planet for Mercury {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 0.2408467)
    }
}
impl Planet for Venus {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 0.61519726)
    }
}
impl Planet for Earth {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 1.0)
    }
}
impl Planet for Mars {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 1.8808158)
    }
}

impl Planet for Jupiter {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 11.862615)
    }
}
impl Planet for Saturn {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 29.447498)
    }
}
impl Planet for Uranus {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 84.016846)
    }
}
impl Planet for Neptune {
    fn years_during(d: &Duration) -> f64 {
        fun_name(d, 164.79132)
    }
}
