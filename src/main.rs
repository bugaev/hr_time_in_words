fn main() {
    println!("Hello, world!");
    // println!("time: {}", num_in_words(23));
    println!("Time in words: {}", timeInWords(5, 59));
}

fn timeInWords(h: i32, m: i32) -> String {
    if m == 0 {
        return num_in_words(h) + " o' clock"
    }
    if m == 30 {
        return "half past ".to_string() + &num_in_words(h);
    }
    if m == 15 {
        return "quarter past ".to_string() + &num_in_words(h);
    }
    if m == 45 {
        return "quarter to ".to_string() + &num_in_words(next_hour(h));
    }
    if m == 1 {
        return "one".to_string() + " minute past " + &num_in_words(h);
    }
    if m == 59 {
        return "one".to_string() + " minute to " + &num_in_words(next_hour(h));
    }
    if m <= 29 {
        return num_in_words(m) + " minutes past " + &num_in_words(h);
    }
    let m = 60 - m;
    return num_in_words(m) + " minutes to " + &num_in_words(next_hour(h));
}

fn next_hour(h: i32) -> i32 {
    if h == 12 {
        return 1;
    } else {
        return h + 1;
    }
}

fn num_in_words(num: i32) -> String {
    match num {
        1 => String::from("one"),
        2 => String::from("two"),
        3 => String::from("three"),
        4 => String::from("four"),
        5 => String::from("five"),
        6 => String::from("six"),
        7 => String::from("seven"),
        8 => String::from("eight"),
        9 => String::from("nine"),
        10 => String::from("ten"),
        11 => String::from("eleven"),
        12 => String::from("twelve"),
        13 => String::from("thirteen"),
        14 => String::from("fourteen"),
        15 => String::from("fifteen"),
        16 => String::from("sixteen"),
        17 => String::from("seventeen"),
        18 => String::from("eighteen"),
        19 => String::from("nineteen"),
        20 => String::from("twenty"),
        21..=29 => String::from("twenty ") + &num_in_words(num - 20), 
        _ => String::from("Unknown")
    }
}
