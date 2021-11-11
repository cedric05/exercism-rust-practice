pub fn abbreviate(phrase: &str) -> String {
    let mut inclusive_take_all = true;

    phrase
        .split_whitespace()
        .into_iter()
        .take_while(|word| {
            // there is no till implementation for iterators
            // mostly doing that will belp me resolve this
            // idea would be to iterate until certain thing happens, inclusive
            if inclusive_take_all && word.ends_with(':') {
                inclusive_take_all = false;
                true
            } else {
                inclusive_take_all
            }
        })
        .flat_map(|x| x.split('-')) // Is this bad? mostly not
        // there won't be many pantuations which needs spliting
        .filter(|word| !word.is_empty())
        .flat_map(|word: &str| {
            let is_all_caps = word == word.to_uppercase(); // if all caps, just prefer first letter
            let ends_with_puntuation = word.ends_with(':'); // using colon and discarding everything else
                                                            // probably better idea would be to discard everything else after ':'
                                                            // than making any more computes
            {
                word.char_indices().filter(move |(index, x)| {
                    match (ends_with_puntuation, is_all_caps, *index) {
                        (true, _, _) => matches!(x, 'A'..='Z'), // if ends with colon, just take all
                        (false, _, 0) => matches!(x, 'a'..='z' | 'A'..='Z'), // take first if it is a alphabet
                        (false, true, _) => false, // take only first if all caps
                        _ => matches!(x, 'A'..='Z'), // else pick all caps
                    }
                })
            }
        })
        .map(|(_, x)| x.to_uppercase())
        .flatten()
        .collect::<String>()
}
