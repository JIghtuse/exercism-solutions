#[derive(Debug, PartialEq)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

pub struct Allergies(pub usize);

impl Allergies {
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        allergen == &Allergen::Eggs
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        vec![]
    }
}
