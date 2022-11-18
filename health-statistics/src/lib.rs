pub struct User {
    name: String,
    age: u32,
    weight: f32,
}

impl User {
    #[must_use]
    pub const fn new(name: String, age: u32, weight: f32) -> Self {
        Self { name, age, weight }
    }

    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    #[must_use]
    pub const fn age(&self) -> u32 {
        self.age
    }

    #[must_use]
    pub const fn weight(&self) -> f32 {
        self.weight
    }

    pub fn set_age(&mut self, new_age: u32) {
        self.age = new_age;
    }

    pub fn set_weight(&mut self, new_weight: f32) {
        self.weight = new_weight;
    }
}
