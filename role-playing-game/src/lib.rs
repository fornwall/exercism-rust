pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    const fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn revive(&self) -> Option<Self> {
        if self.is_alive() {
            return None;
        }
        Some(Self {
            health: 100,
            mana: (self.level >= 10).then_some(100),
            level: self.level,
        })
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        match self.mana {
            Some(ref mut value) if *value >= mana_cost => {
                *value -= mana_cost;
                mana_cost * 2
            }
            Some(_) => 0,
            None => {
                self.health = self.health.saturating_sub(mana_cost);
                0
            }
        }
    }
}
