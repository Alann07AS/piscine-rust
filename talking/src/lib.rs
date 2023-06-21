pub fn talking(text: &str) -> &str {
    if text.len() == 0 {
        return "Just say something!";
    }
    let is_up = text.chars().all(|ch| !ch.is_ascii_alphabetic() || ch.is_ascii_uppercase())
                    & text.chars().any(|ch| ch.is_ascii_alphabetic());
    let is_intero = text.chars().any(|ch| ch == '?');


    match (is_up, is_intero) {
        (true, false) => "There is no need to yell, calm down!" ,
        (false, true) => "Sure." ,
        (true, true) => "Quiet, I am thinking!" ,
        (false, false) =>  "Interesting",
    }
}
//"There is no need to yell, calm down!"
//"Sure."
//"Quiet, I am thinking!"
//"Interesting"
//"Just say something!"


