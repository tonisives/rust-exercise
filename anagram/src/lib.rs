use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut res: HashSet<&str> = HashSet::new();

    possible_anagrams.iter().for_each(|w| {
        if is_anagram(word, w) {
            res.insert(w);
        }
    });

    res
}

pub fn is_anagram(a: &str, b: &str) -> bool {
    // how to find anagram? sort both of the strings and see if they match

    let mut a_arr: Vec<char> = a.chars().collect();
    let mut b_arr: Vec<char> = b.chars().collect();

    if a_arr == b_arr {
        false
    } else {
        a_arr.sort();
        b_arr.sort();
        a_arr == b_arr
    }
}
