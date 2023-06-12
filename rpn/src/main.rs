// level 4: rpn (Alann)

fn main() {
    let args: Vec < String > = std::env::args().collect();
    if args.len() != 2 {
        println!("Error");
        return;
    };

    rpn(&args[1]);
}

fn rpn(calc: &String) {

    let calc: Vec<&str> = calc.split_whitespace().collect();

    let mut count_op_num = (0, 0);
    let mut is_err = false;


    let mut values: Vec < i64 > = vec![];

    let mut last_values: Vec < i64 > = vec![];


    calc.into_iter().for_each(|el| {

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
                    is_err = true;
                    return;
                };
                
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
                count_op_num.0 += 1;
            }
        };
    });

    if is_err || count_op_num.0 + 1 != count_op_num.1 {
        println!("Error");
        return;
    };
    
    println!("{}", values[1]);
}