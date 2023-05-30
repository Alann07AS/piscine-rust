pub fn delete_and_backspace(s: &mut String) {
    delete_before(s, '-');
    *s = s.chars().rev().collect::<String>();
    delete_before(s, '+');
    *s = s.chars().rev().collect::<String>();
}

fn delete_before(s: &mut String, triger: char) {
    let mut new_s: String = "".to_string(); 
    for ch in s.chars() {
        if ch != triger {new_s.push(ch);} else {new_s.pop();}
    }
    *s = new_s;
}

pub fn do_operations(v: &mut Vec<String>) {
    let mut is_sum: bool = false;
    // let mut operant: Vec<i32> = vec![];
    for op in v {
        let mut split: Vec<&str> = op.split("+").collect();
        let mut result: i32 = 0;
        is_sum = true;
        if split.len() != 2 {
            split = op.split("-").collect();
            is_sum = false;
        }
        let op1: i32 = split[0].parse().unwrap();
        let op2: i32 = split[1].parse().unwrap();
        if is_sum {
            result = op1 + op2
        } else {
            result = op1 - op2
        }
        *op = result.to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut st = String::from("bpp--o+er+++sskroi-++lcw");
        delete_and_backspace(&mut st);
        assert_eq!(st, String::from("borrow"));
    }
    #[test]
    fn test() {
        let mut v = vec!["2+2".to_string(), "3+2".to_string(), "10-3".to_string(), "5+5".to_string()];
        do_operations(&mut v);
        assert_eq!(vec!["4".to_string(), "5".to_string(), "7".to_string(), "10".to_string()], v);
    }
}
