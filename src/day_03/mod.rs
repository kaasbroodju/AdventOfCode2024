use std::str::Chars;

pub fn first_part() {
    let mut input =include_str!("input.txt")
        .chars();

    let mut sum = 0;
    // flags
    let mut left_side = String::with_capacity(3);
    let mut right_side = String::with_capacity(3);
    while let Some(char) = input.next() {

        if char != 'm' {continue;}
        if !matches!(input.next(), Some('u')) {continue;}
        if !matches!(input.next(), Some('l')) {continue;}
        if !matches!(input.next(), Some('(')) {continue;}

        let mut i = 0usize;
        let mut char = input.next();
        while matches!(char, Some(c) if c.is_ascii_digit()) {
            left_side.push(char.unwrap());
            i += 1;
            char = input.next();
        }
        // println!("{:?}", input.next());
        // println!("{left_side}");
        if !matches!(char, Some(',')) || i == 0 {left_side.clear();continue;}
        // println!("{left_side}");
        char = input.next();
        i = 0usize;
        while matches!(char, Some(c) if c.is_ascii_digit()) {
            right_side.push(char.unwrap());
            i += 1;
            char = input.next();
        }
        if !matches!(char, Some(')')) || i == 0 {left_side.clear();right_side.clear();continue;}
        // println!("{left_side} * {right_side}");
        sum += left_side.parse::<i32>().unwrap() * right_side.parse::<i32>().unwrap();
        left_side.clear();right_side.clear();
    }

    // println!("{}", sum);
}

pub fn first_part_state_machine() {
    let input = include_str!("input.txt").as_bytes();
    let mut results = 0usize;
    let mut state = 0;
    let mut left_number = 0usize;
    let mut right_number = 0usize;
    let mut current_num = 0usize;

    for &byte in input {
        // Determine state transitions branchlessly
        state = match state {
            0 => ((byte == b'm') as u8) * 1, // Start of "mul("
            1 => ((byte == b'u') as u8) * 2, // Next char in "mul("
            2 => ((byte == b'l') as u8) * 3, // Next char in "mul("
            3 => ((byte == b'(') as u8) * 4, // Open parenthesis
            4 => {
                if byte >= b'0' && byte <= b'9' {
                    current_num = current_num * 10 + (byte - b'0') as usize;
                    4 // Continue reading first number
                } else if byte == b',' {
                    left_number = current_num;
                    current_num = 0;
                    5 // Move to reading second number
                } else {
                    left_number = 0;
                    current_num = 0;
                    0 // Reset on invalid input
                }
            }
            5 => {
                if byte >= b'0' && byte <= b'9' {
                    current_num = current_num * 10 + (byte - b'0') as usize;
                    5 // Continue reading second number
                } else if byte == b')' {
                    right_number = current_num;
                    // println!("{left_number} * {right_number} == {current_num}");
                    results += left_number * right_number;
                    // current_num = 0;
                    left_number = 0;
                    right_number = 0;
                    current_num = 0;
                    0 // Reset after closing parenthesis
                } else {
                    left_number = 0;
                    right_number = 0;
                    current_num = 0;
                    0 // Reset on invalid input
                }
            }
            _ => panic!(), // Default state
        };
    }

    // println!("{}", results);
}

pub fn second_part() {
    let mut input =include_str!("input.txt")
        .chars();

    let mut sum = 0;
    let mut is_enabled = true;
    // flags
    let mut left_side = String::with_capacity(3);
    let mut right_side = String::with_capacity(3);
    while let Some(char) = input.next() {

        if char == 'm' && is_enabled {
            if handle_mul(&mut input, &mut sum, &mut left_side, &mut right_side) { continue; }
        } else if char == 'd' {
            if !matches!(input.next(), Some('o')) { continue; }
            match input.next() {
                // None => {continue;}
                Some(char) if char == '(' => {
                    if !matches!(input.next(), Some(')')) { continue; }
                    is_enabled = true;
                }
                Some(char) if char == 'n' => {
                    if !matches!(input.next(), Some('\'')) { continue; }
                    if !matches!(input.next(), Some('t')) { continue; }
                    if !matches!(input.next(), Some('(')) { continue; }
                    if !matches!(input.next(), Some(')')) { continue; }
                    is_enabled = false;
                }
                _ => {continue;}
            };
        }


    }

    println!("{}", sum);
}

fn handle_mul(input: &mut Chars, sum: &mut i32, left_side: &mut String, right_side: &mut String) -> bool {
    if !matches!(input.next(), Some('u')) { return true; }
    if !matches!(input.next(), Some('l')) { return true; }
    if !matches!(input.next(), Some('(')) { return true; }

    let mut i = 0usize;
    let mut char = input.next();
    while matches!(char, Some(c) if c.is_ascii_digit()) {
        left_side.push(char.unwrap());
        i += 1;
        char = input.next();
    }
    // println!("{:?}", input.next());
    // println!("{left_side}");
    if !matches!(char, Some(',')) || i == 0 {
        left_side.clear();
        return true;
    }
    // println!("{left_side}");
    char = input.next();
    i = 0usize;
    while matches!(char, Some(c) if c.is_ascii_digit()) {
        right_side.push(char.unwrap());
        i += 1;
        char = input.next();
    }
    if !matches!(char, Some(')')) || i == 0 {
        left_side.clear();
        right_side.clear();
        return true;
    }
    // println!("{left_side} * {right_side}");
    *sum += left_side.parse::<i32>().unwrap() * right_side.parse::<i32>().unwrap();
    left_side.clear();
    right_side.clear();
    false
}

pub fn second_part_state_machine() {
    let input = include_str!("input.txt").as_bytes();
    let mut results = 0usize;
    let mut state = 0;
    let mut left_number = 0usize;
    let mut right_number = 0usize;
    let mut current_num = 0usize;
    let mut is_enabled = true;

    for &byte in input {
        // Determine state transitions branchlessly
        state = match state {
            0 => ((byte == b'm' && is_enabled) as u8) * 1 + ((byte == b'd') as u8) * 10, // Start of "mul("
            // mul(
            1 => ((byte == b'u') as u8) * 2, // Next char in "mul("
            2 => ((byte == b'l') as u8) * 3, // Next char in "mul("
            3 => ((byte == b'(') as u8) * 4, // Open parenthesis
            // number
            4 => {
                if byte >= b'0' && byte <= b'9' {
                    current_num = current_num * 10 + (byte - b'0') as usize;
                    4 // Continue reading first number
                } else if byte == b',' {
                    left_number = current_num;
                    current_num = 0;
                    5 // Move to reading second number
                } else {
                    left_number = 0;
                    current_num = 0;
                    0 // Reset on invalid input
                }
            }
            5 => {
                if byte >= b'0' && byte <= b'9' {
                    current_num = current_num * 10 + (byte - b'0') as usize;
                    5 // Continue reading second number
                } else if byte == b')' {
                    right_number = current_num;
                    // println!("{left_number} * {right_number}");
                    results += left_number * right_number;
                    // current_num = 0;
                    left_number = 0;
                    right_number = 0;
                    current_num = 0;
                    0 // Reset after closing parenthesis
                } else {
                    left_number = 0;
                    right_number = 0;
                    current_num = 0;
                    0 // Reset on invalid input
                }
            }

            10 => ((byte == b'o') as u8) * 11,
            11 => ((byte == b'(') as u8) * 12 + ((byte == b'n') as u8) * 20,

            // do()
            12 => {
                if byte == b')' {
                    is_enabled = true;
                }
                0
            },
            // don't()
            20 => ((byte == b'\'') as u8) * 21,
            21 => ((byte == b't') as u8) * 22,
            22 => ((byte == b'(') as u8) * 23,
            23 => {
                if byte == b')' {
                    is_enabled = false;
                }
                0
            },
            _ => panic!(), // Default state
        };
    }

    println!("{}", results);
}