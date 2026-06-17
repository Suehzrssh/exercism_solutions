use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // 1. Lowercase the target word once
    let word_lower: String = word.to_lowercase();
    
    // 2. Sort the characters of the lowercase target word for easy comparison
    let mut word_sorted: Vec<char> = word_lower.chars().collect();
    word_sorted.sort_unstable();

    possible_anagrams
        .iter()
        .copied() // Converts &&str to &str
        .filter(|&candidate| {
            let candidate_lower = candidate.to_lowercase();
            
            // Condition 1: A word is not an anagram of its identical self
            if candidate_lower == word_lower {
                return false;
            }

            // Condition 2: Must contain the exact same characters
            let mut candidate_sorted: Vec<char> = candidate_lower.chars().collect();
            candidate_sorted.sort_unstable();
            
            candidate_sorted == word_sorted
        })
        .collect() // Collects matching &'a str into a HashSet
}
