pub fn spell(mut n: u64) -> String {
    if n == 0 {
        return "zero".to_owned()
    }

    let mut result = vec![];
    let milion = n/1000000;
    if milion > 0 {
        result.push(unit(milion));
        result.push(lvl(n));
    }

    n = n%1000000;

    let milier = n/1000;
    if milier > 0 {
        let m = transform_100(milier);
        if m != "" {
            result.push(m);
            result.push(lvl(1000));    
        }
    }

    n = n%1000;

    let cent = n/1;
    if cent > 0 {
        result.push(transform_100(cent));
    }

    result.join(" ")
}

pub fn transform_100(n: u64) -> String {
    let mut result = "".to_string();
    let c = n/100;
    if c > 0 {
        result += &unit(c);
        result += &lvl(100);
    }
    let d = &transform_10(n%100);
    if d != "" {
        result += d;
    }
    result
}

pub fn transform_10(n: u64) -> String {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        20 => "Twenty",
        21 => "Twenty-one",
        22 => "Twenty-two",
        23 => "Twenty-three",
        24 => "Twenty-four",
        25 => "Twenty-five",
        26 => "Twenty-six",
        27 => "Twenty-seven",
        28 => "Twenty-eight",
        29 => "Twenty-nine",
        30 => "Thirty",
        31 => "Thirty-one",
        32 => "Thirty-two",
        33 => "Thirty-three",
        34 => "Thirty-four",
        35 => "Thirty-five",
        36 => "Thirty-six",
        37 => "Thirty-seven",
        38 => "Thirty-eight",
        39 => "Thirty-nine",
        40 => "Forty",
        41 => "Forty-one",
        42 => "Forty-two",
        43 => "Forty-three",
        44 => "Forty-four",
        45 => "Forty-five",
        46 => "Forty-six",
        47 => "Forty-seven",
        48 => "Forty-eight",
        49 => "Forty-nine",
        50 => "Fifty",
        51 => "Fifty-one",
        52 => "Fifty-two",
        53 => "Fifty-three",
        54 => "Fifty-four",
        55 => "Fifty-five",
        56 => "Fifty-six",
        57 => "Fifty-seven",
        58 => "Fifty-eight",
        59 => "Fifty-nine",
        60 => "Sixty",
        61 => "Sixty-one",
        62 => "Sixty-two",
        63 => "Sixty-three",
        64 => "Sixty-four",
        65 => "Sixty-five",
        66 => "Sixty-six",
        67 => "Sixty-seven",
        68 => "Sixty-eight",
        69 => "Sixty-nine",
        70 => "Seventy",
        71 => "Seventy-one",
        72 => "Seventy-two",
        73 => "Seventy-three",
        74 => "Seventy-four",
        75 => "Seventy-five",
        76 => "Seventy-six",
        77 => "Seventy-seven",
        78 => "Seventy-eight",
        79 => "Seventy-nine",
        80 => "Eighty",
        81 => "Eighty-one",
        82 => "Eighty-two",
        83 => "Eighty-three",
        84 => "Eighty-four",
        85 => "Eighty-five",
        86 => "Eighty-six",
        87 => "Eighty-seven",
        88 => "Eighty-eight",
        89 => "Eighty-nine",
        90 => "Ninety",
        91 => "Ninety-one",
        92 => "Ninety-two",
        93 => "Ninety-three",
        94 => "Ninety-four",
        95 => "Ninety-five",
        96 => "Ninety-six",
        97 => "Ninety-seven",
        98 => "Ninety-eight",
        99 => "Ninety-nine",
        _ => "",
    }.to_lowercase()
}


fn lvl(n: u64) -> String {
    match n {
        1000000..=9999999 => "million",
        1000..=100000 => "thousand",
        100..=999 => "hundred",
        _ => ""
    }.to_string().to_lowercase()
}

fn unit(n: u64) -> String {
    match n {
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        _ => ""
    }.to_string().to_lowercase()
}

fn diz(n: u64) -> String {
    let a = &fuckin11_19(n);
    match n {
        10 => "Ten",
        10..=19 => a,
        20..=29 => "Twenty",
        30..=39 => "Thirty",
        40..=49 => "Forty",
        50..=59 => "Fifty",
        60..=69 => "Sixty",
        70..=79 => "Seventy",
        80..=89 => "Eighty",
        90..=99 => "Ninety",
        _ => ""
    }.to_lowercase()
}

fn fuckin11_19(n: u64) -> String {
    match n {
        11 => "Eleven",
        12 => "Twelve",
        13 => "Thirteen",
        14 => "Fourteen",
        15 => "Fifteen",
        16 => "Sixteen",
        17 => "Seventeen",
        18 => "Eighteen",
        19 => "Nineteen",
        _ => ""
    }.to_string().to_lowercase() 
}


// 100 = One hun