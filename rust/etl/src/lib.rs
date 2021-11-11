use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(index, s)| {
            s.iter() //
                .map(|c| (c.to_ascii_lowercase(), *index))
        })
        .collect::<BTreeMap<char, i32>>()
}
