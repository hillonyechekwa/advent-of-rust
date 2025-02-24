const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;

pub struct SnowKg(pub f64);

impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}

pub struct SnowLb(pub f64);

impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}

pub struct Snowball(pub i64);

impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}

impl From<SnowKg> for Snowball {
    // 1. Implement the conversion from SnowKg to Snowball
    fn from (snowkg: SnowKg) -> Self {
        let value: f64 = (snowkg.0 / SNOWBALL_WEIGHT_KG).round();
        Snowball(value as i64)
    }
}

// 2. Implement the same for SnowLb
impl From<SnowLb> for Snowball{
    fn from (snowlb: SnowLb) -> Self {
        let value: f64 = (snowlb.0 / SNOWBALL_WEIGHT_LB).round();
        Snowball(value as i64)
    }
}
