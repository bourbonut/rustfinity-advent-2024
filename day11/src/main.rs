use std::{cmp::Ordering, error::Error, ops::Deref};

#[derive(Debug, Clone)]
pub struct Location {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub area: f64,
    pub snow: Snowball,
}

impl Location {
    // 1. Implement the `new()` method.
    // Parameters (must be in order):
    // - x: f64
    // - y: f64
    // - z: f64
    // - area: f64
    // - snow: Either `SnowKg`, `SnowLb` or `Snowball`
    pub fn new(x: f64, y: f64, z: f64, area: f64, snow: impl Into<Snowball>) -> Self {
        Location { x, y, z, area, snow: snow.into() }
    }

    pub fn density(&self) -> f64 {
        // 2. Implement the `density()` method.
        // Calculation: snow / area
        // all area is in one unit, so don't worry about the unit conversion.
        // Return 0.0 if the area is 0.0.
        if self.area == 0.0 {
            return 0.0;
        }
        *self.snow as f64 / self.area
    }
}

pub fn find_best_location(locations: Vec<Location>) -> Result<Location, Box<dyn Error>> {
    // 3. Find the location with the highest snow density.
    locations.into_iter().max_by(|x, y| x.density().partial_cmp(&y.density()).unwrap_or(Ordering::Equal)).ok_or("Not valid locations".into())
}

const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

#[derive(Debug)]
pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

impl Deref for SnowKg {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug)]
pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

impl Deref for SnowLb {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl Deref for Snowball {
    type Target = i64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
        Snowball(snowballs)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
        Snowball(snowballs)
    }
}

fn main() {
    let snowball1 = Snowball(20);
    let snowball2 = SnowKg(23.);
    let snowball3 = SnowLb(129.);
    let snowball4 = Snowball(80);
    let locations = vec![ 
        Location::new(0.1, 0.1, 0.1, 20., snowball1),
        Location::new(0.1, 0.1, 0.1, 0., snowball2),
        Location::new(0.1, 0.1, 0.1, 100., snowball3),
        Location::new(0.1, 0.1, 0.1, 10., snowball4),
    ];
    println!("{:?}", find_best_location(locations));
    println!("{:?}", find_best_location(vec![]));
}
