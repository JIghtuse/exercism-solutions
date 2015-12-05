#[derive(Copy, Clone, Debug, PartialEq)]
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
    pub fn is_allergic_to(&self, allergen: Allergen) -> bool {
        allergen as usize & self.0 != 0
    }
    pub fn allergies(&self) -> Vec<Allergen> {
        let mut all: Vec<Allergen> = vec![Allergen::Eggs,
                                          Allergen::Peanuts,
                                          Allergen::Shellfish,
                                          Allergen::Strawberries,
                                          Allergen::Tomatoes,
                                          Allergen::Chocolate,
                                          Allergen::Pollen,
                                          Allergen::Cats];
        all.retain(|&x| self.is_allergic_to(x));
        all
    }
}
