pub fn insertion_sort(slice: &mut [i32], steps: usize) {
    for i in 1..=steps {
        let mut j = i;
        while j > 0 && slice[j] < slice[j - 1] {
            slice.swap(j, j - 1);
            j = j - 1;
        }
    }
}



// level 4: rpn (Alann)

fn main() {
    let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		print!("{}\n", "Error".to_string());
		return;
	};
	//print!("{:?}", args[1..2]);

    rpn(&args[1]);
	//print!("{}", "A".to_string());
}

fn rpn(calc: &String) {
	let calc_copy = calc.clone();
	
	let calc: Vec<&str> = calc.split_whitespace().collect();

	let mut count_op_num = (0, 0);
	let mut is_err = false;
	
	
	let mut values: Vec<i64> = vec![];
	
	let mut last_values: Vec<i64> = vec![];
	
	let mut step_count = 0; 
	
	calc.into_iter().for_each(|el| {
		
		step_count += 1;
		if is_err {
			return;
		}
		match el.parse() {
			Ok(nb) => {
				values.push(nb);
				if values.len() > 2 {
					last_values.push(values[0]);
					values.remove(0);
				};
				count_op_num.1 += 1;
			},
			Err(_) => {
				if values.len() != 2 {
					//print!("A{:?}    {}    {:?}", calc_copy, step_count, values.len());
					
					//if step_count != calc.len()-2 {
						is_err = true;
					
					return;
					
				};
				//println!("{}   {}   {}   ", el, values[0] , values[1]);
				match el {
				"+" => {
					//last_value = values[1];
					values[1] = values[0] + values[1];
					values[0] = last_values.pop().unwrap_or(0);
					//values.remove(1);
				},
				"-" => {
					//last_value = values[1];
					values[1] = values[0] - values[1];
					values[0] = last_values.pop().unwrap_or(0);
					//values.remove(1);
				},
				"*" => {
					//last_value = values[1];
					values[1] = values[0] * values[1];
					values[0] = last_values.pop().unwrap_or(0);
					//values.remove(1);
				},
				"/" => {
					//last_value = values[1];
					values[1] = values[0] / values[1];
					values[0] = last_values.pop().unwrap_or(0);
					//values.remove(1);
				},
				"%" => {
					//last_value = values[1];
					values[1] = values[0] % values[1];
					values[0] = last_values.pop().unwrap_or(0);
					//values.remove(1);
				},
				&_ => {
					is_err = true;
				},
			};
				//println!("=====  {}   ",values[1]);
				count_op_num.0 += 1;
			}
		};
	});
	
	if !(count_op_num.0 + 1 == count_op_num.1) {
		is_err = true;
		//print!("nEGAL___________________________");
		
	};
	//print!("{:?}", count_op_num);
	
	//print!("{}", "values[0]");
	if is_err {
		println!("{}", "Error".to_string());
		return;
	};
	if values.len() > 0 {
		println!("{}", values[1]);
	} else {
		println!("{}", "Error");
	}
	
	if values[1] == 250 {
		print!("___A{:?} ____", calc_copy);
	}
}