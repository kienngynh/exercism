use std::collections::HashSet;
pub fn sort(word: &str) -> Vec<char> {
    let mut vec_word:Vec<char> = word.to_lowercase().as_str().chars().collect();
    vec_word.sort_unstable();
    vec_word
}
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set_anagrams: HashSet<&str> = HashSet::new();
    let vec_word = sort(word);
    for s in possible_anagrams.iter() {
        let vec_s = sort(s);
        if s.to_lowercase() != word.to_lowercase() && vec_s == vec_word {
            set_anagrams.insert(s);
        }
    }
    set_anagrams
}
