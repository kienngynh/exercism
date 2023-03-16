use std::collections::HashSet;
pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set_anagrams: HashSet<&str> = HashSet::new();
    let lowcase_word = word.to_lowercase();
    let mut vec_word: Vec<u8> = lowcase_word.as_bytes().to_vec();
    vec_word.sort();
    for s in possible_anagrams.iter() {
        let lowcase_s = s.to_lowercase();
        let mut vec_s: Vec<u8> = lowcase_s.as_bytes().to_vec();
        vec_s.sort();
        if lowcase_s != lowcase_word && vec_s == vec_word {
            set_anagrams.insert(s);
        }
    }
    set_anagrams
}
fn main() {
    let word = "allergy";
    let inputs = [
        "gallery",
        "ballerina",
        "regally",
        "clergy",
        "largely",
        "leading",
    ];
    let set = anagrams_for(word, &inputs);
    println!("{:?}", set);
}
