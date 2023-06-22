pub fn pig_latin(text: &str) -> String {
    let mut result = "".to_string();
    match text.chars().nth(0).unwrap().to_ascii_lowercase() {
        'a'|'e'|'i'|'o'|'u' => {result += text},
        _ => {
            if &text[1..3] == "qu" {
                result = result + &text[3..] + &text[..3]
            } else {
                for (i, ch) in text.chars().enumerate() {
                    match ch {
                    'a'|'e'|'i'|'o'|'u' =>  {result = result + &text[i..] + &text[..i]; break},
                    _ => {},
                    }  
                }
            }
        },
    }
    result + "ay"
}