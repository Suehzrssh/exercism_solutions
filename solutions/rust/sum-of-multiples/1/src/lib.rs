use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut multiplies = HashSet::new();

    for &f in factors {
        if f == 0 { continue; }

        let mut current_multiple = f;
        while current_multiple < limit {
            multiplies.insert(current_multiple);
            current_multiple += f;
        }
    }

multiplies.iter().sum()
}
