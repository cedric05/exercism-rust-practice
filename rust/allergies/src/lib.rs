pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergen {
    pub fn score(&self) -> u32 {
        match self {
            Allergen::Eggs => 2u32.pow(0),
            Allergen::Peanuts => 2u32.pow(1),
            Allergen::Shellfish => 2u32.pow(2),
            Allergen::Strawberries => 2u32.pow(3),
            Allergen::Tomatoes => 2u32.pow(4),
            Allergen::Chocolate => 2u32.pow(5),
            Allergen::Pollen => 2u32.pow(6),
            Allergen::Cats => 2u32.pow(7),
        }
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        let score = allergen.score();
        self.score & score == score
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        [
            Allergen::Cats,
            Allergen::Chocolate,
            Allergen::Eggs,
            Allergen::Peanuts,
            Allergen::Pollen,
            Allergen::Shellfish,
            Allergen::Strawberries,
            Allergen::Tomatoes,
        ]
        .into_iter()
        .filter(|all| self.is_allergic_to(all))
        .collect()
    }
}
