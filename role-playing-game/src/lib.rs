pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub const fn revive(&self) -> Option<Self> {
        let is_alive = self.health > 0;
        if is_alive {
            return None;
        }
        Some(Self {
            health: 100,
            mana: if self.level >= 10 { Some(100) } else { None },
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
