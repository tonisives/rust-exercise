pub struct Allergies {
    score: u32,
}

#[derive(Debug, PartialEq, Eq)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        todo!("Given the '{score}' score, construct a new Allergies struct.");
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        todo!("Determine if the patient is allergic to the '{allergen:?}' allergen.");
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        // TODO:
        // for loop starts from the top.
        // tests all numbers, finds first with modulo 0 and <= curr. adds as allergen
        // removes this from the curr, continues the loop until curr = 0
        // if have remainder without allergen, then ignore.

        let curr = self.score;
        let mut allergens: Vec<Allergen> = vec![];

        for i in self.score..0 {}

        // todo!(
        //     "Return the list of allergens contained within the score with which the Allergies struct was made."
        // );

        vec![]
    }
}
