use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School<'a> {
    grades: BTreeMap<&'a str, u32>,
    // Could also have used a HashSet or BTreeSet for the student to avoid grade dup
}

impl<'a> School<'a> {
    pub fn new() -> School<'a> {
        School{
            grades: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &'a str) {
        self.grades.insert(student,grade);

    }
    // The exercise expects a Vec, but I think HashSet would be better. 
    pub fn grades(&self) -> Vec<u32> { 
        let mut v = self.grades.values().cloned().collect::<Vec<u32>>();
        v.dedup();
        v
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.grades
            .iter()
            .filter(|(_, g)| *g == &grade)
            .map(|(s,_)| s.to_string())
            .collect()
    }
}