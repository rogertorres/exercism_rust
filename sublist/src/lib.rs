#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    match (first.len(), second.len()){
        // If they're both empty => equal
        (0, 0) => Comparison::Equal,
        // If second is empty, it is superlsit
        (_,0) => Comparison::Superlist,
        // If first is empty, it is sublist
        (0,_) => Comparison::Sublist,
        // If neither are empty, check wich one is bigger and if it contains the other's elements
        // If it does, that one is superlist, otherwise, they're simply unequal
        // Windows works because in this exercise elements are groupe equally in both arrays
        (f,s) if f > s => {
            if first.windows(s).any(|f_w| f_w == second) {
                Comparison::Superlist
            } else {
                Comparison::Unequal
            }
        },
        (f,s) if f < s => {
            if second.windows(f).any(|s_w| s_w == first) {
                Comparison::Sublist
            } else {
                Comparison::Unequal
            }
        },
        (_, _) => if first == second {Comparison::Equal} else {Comparison::Unequal},
    }
}

// What if not all the list items were contiguous? No windows()...
pub fn sublist_mixed<T: PartialEq>(first: &[T], second: &[T]) -> Comparison {
    todo!()
}