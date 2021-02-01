extern crate enum_iterator;
use std::collections::BTreeMap;
use enum_iterator::IntoEnumIterator;

pub struct Allergies(Vec<Allergen>);

#[derive(Debug, IntoEnumIterator, PartialEq, Clone)]
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

impl Allergies {
    pub fn new(score: u32) -> Self {
        // Set fixed values for allergies 
        let mut map: BTreeMap<u32, Allergen> = BTreeMap::new();
        let mut p = 0;
        Allergen::into_enum_iter().for_each(|x| {
            map.insert(2u32.pow(p), x);
            p += 1;
        });

        // Push allergies to return vector from highest to lower values
        let mut temp = score;
        let mut result: Vec<Allergen> = vec![];
        while let Some((key, val)) = map.iter().filter(|(&k,_)| k <= temp).next_back(){ 
            temp -= key;
            result.push(val.clone());
        };

        // I chose to dedup, but perhaps a check before insert 
        // or a different way to iterate would be better
        result.dedup();
        Allergies(result)
    }

    // Check if a certain Allergen is within the vector
    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.0.contains(allergen)
    }

    // Return Allergen vector. It is cloning because the exercise didn't expected a ref
    pub fn allergies(&self) -> Vec<Allergen> {
        self.0.clone()
    }
}
