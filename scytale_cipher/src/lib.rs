pub fn scytale_cipher(message: String, i: u32) -> String {
    let i = i as usize;
    let mut spires: Vec<Vec<char>> = vec![vec![]];
    let mut spires_index = 0;
    message.chars().into_iter().for_each(|ch|{
        if spires[spires_index].len() != i {
            spires[spires_index].push(ch);
        } else {
            spires.push(vec![]);
            spires_index+=1;
            spires[spires_index].push(ch);
        }
    });

    while spires[spires_index].len() != i {
        spires[spires_index].push(' ');
    };
    
    println!("{:?}", spires);

    let mut result = "".to_string();
    for i in 0..i {
        for ele in &spires {
            result.push(ele[i])
        }
    }
    result.trim().to_string()
}