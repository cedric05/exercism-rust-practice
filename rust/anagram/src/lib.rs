#![feature(is_sorted)]
use std::{collections::HashSet, hash::Hash};

trait Anagaram {
    fn is_anagram(&self, other: &Self) -> bool;
}

fn fun_name(str: &str) -> Vec<u8> {
    let mut original = str
        .to_lowercase()
        .chars()
        .map(|a| a as u8)
        .collect::<Vec<u8>>();
    original.sort_unstable_by_key(|a| *a);
    original
}
impl Anagaram for str {
    fn is_anagram(&self, other: &Self) -> bool {
        if self.to_lowercase() == other.to_lowercase() {
            return false;
        } else if fun_name(&self) == fun_name(other) {
            return true;
        }
        false
    }
}

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|a| a.is_anagram(word))
        .cloned()
        .collect::<HashSet<&'a str>>()
}
