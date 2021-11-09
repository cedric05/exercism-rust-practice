const WHATEVER: &str = "Whatever.";
const ASKING_QUESTION_NOT_YELLING: &str = "Sure.";
const ASKING_QUESTION_WITH_YELLING: &str = "Calm down, I know what I'm doing!";
const JUST_YELLING_NO_QUESTION: &str = "Whoa, chill out!";
const ONLY_WHIESPACE: &str = "Fine. Be that way!";
pub fn reply(msg: &str) -> &str {
    match msg.trim() {
        m if m.is_empty() => ONLY_WHIESPACE,
        m if is_yelling(m) => {
            if m.ends_with('?') {
                ASKING_QUESTION_WITH_YELLING
            } else {
                JUST_YELLING_NO_QUESTION
            }
        }
        m if m.ends_with('?') => ASKING_QUESTION_NOT_YELLING,
        _ => WHATEVER,
    }
}

fn is_yelling(m: &str) -> bool {
    let have_letters = m.chars().filter(|x| x.is_alphabetic()).count() > 0;
    have_letters && m.to_uppercase() == m
}
