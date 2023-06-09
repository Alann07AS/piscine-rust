use std::collections::HashMap;

pub fn counting_words(words: &str) -> HashMap<String, u32> {
	let mut la_map = HashMap::new();
	words.replace(&['(', ')', '“', '”', ',', '\"', '.', ';', '!', ':', '―'][..], "")
	.replace(" '", "")
	.replace("' ", "")
		.split_whitespace().into_iter().for_each(|wd| {
		*la_map.entry(wd.to_string().to_lowercase()).or_insert(0)+=1;
	});
	la_map
}