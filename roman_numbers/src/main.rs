use roman_numbers::RomanNumber;

fn main() {
	println!("32  -> {:?}", RomanNumber::from(32));
	println!("9  -> {:?}", RomanNumber::from(9));
	println!("45  -> {:?}", RomanNumber::from(45));
	println!("0  -> {:?}", RomanNumber::from(0));
}