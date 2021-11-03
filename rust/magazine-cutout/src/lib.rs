// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::{collections::HashMap, slice::SliceIndex};

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut mp: HashMap<&str, u8> = HashMap::new();
    magazine.iter().for_each(|a| {
        match mp.get(*a) {
            Some(count) => mp.insert(*a, count + 1),
            None => mp.insert(*a, 1),
        };
    });

    for n in note {
        if let Some(cnt) = mp.get_mut(*n) {
            if (cnt > &mut 0) {
                *cnt -= 1;
            } else {
                return false;
            }
        } else {
            return false;
        }
    }
    true
}
