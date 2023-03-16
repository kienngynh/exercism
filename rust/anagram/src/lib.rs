use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set_anagrams: HashSet<&str> = HashSet::new();
    let set_word:HashSet<char> = word.chars().map(|x|x.to_ascii_lowercase()).collect();
    for s in possible_anagrams.iter() {
        if s != &word && (set_word == s.chars().map(|x|x.to_ascii_lowercase()).collect()) {
            set_anagrams.insert(s);
        }
    }
    set_anagrams
}