use commits_stats::{commits_per_week, commits_per_author};
use std::fs;
use chrono::{ DateTime};


fn main() {
	let contents = fs::read_to_string("commits.json").unwrap();
	let serialized = json::parse(&contents).unwrap();
	println!("{:?}", commits_per_week(&serialized));
	// println!("{:?}", commits_per_author(&serialized));
}

// fn main() {
    // // Créez une date correspondant au premier jour de la semaine
	// let date = DateTime::parse_from_rfc3339("2020-12-10T08:26:02Z").unwrap();
// 
    // // Formatez la date en utilisant le format "YYYY-'W'WW"
    // let formatted_string = date.format("%Y-W%U").to_string();
// 
    // println!("{}", formatted_string); // Affiche "2020-W1"
// }