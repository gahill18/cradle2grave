type Skillpoints = u8;
#[derive(Debug)]
pub struct Skills {
    diplomacy: Skillpoints,
    intrigue: Skillpoints,
    learning: Skillpoints,
    martial: Skillpoints,
    prowess: Skillpoints,
    stewardship: Skillpoints,
}

impl Skills {
    pub fn default() -> Skills {
        Skills::new(5, 5, 5, 5, 5, 5)
    }
    pub fn new(
        d: Skillpoints,
        i: Skillpoints,
        l: Skillpoints,
        m: Skillpoints,
        p: Skillpoints,
        s: Skillpoints,
    ) -> Self {
        Self {
            diplomacy: d,
            intrigue: i,
            learning: l,
            martial: m,
            prowess: p,
            stewardship: s,
        }
    }
}
