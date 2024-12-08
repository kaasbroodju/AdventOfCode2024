use std::time::Instant;

fn is_calculation_possible(current_amount: usize, target: usize, stack: &[usize]) -> bool {
    if current_amount > target { return false; }
    let is_empty = stack.len() == 0;
    if current_amount == target && is_empty { return true; }
    if is_empty { return false; }
    if is_calculation_possible(current_amount + stack[0], target, &stack[1..]) {
        return true;
    }

    if is_calculation_possible(current_amount * stack[0], target, &stack[1..]) {
        return true;
    }

    false
}

fn is_calculation_possible_v2(current_amount: usize, target: usize, stack: &[usize]) -> bool {
    if current_amount > target { return false; }
    let is_empty = stack.len() == 0;
    if current_amount == target && is_empty { return true; }
    if is_empty { return false; }
    if is_calculation_possible_v2(current_amount + stack[0], target, &stack[1..]) {
        return true;
    }

    if is_calculation_possible_v2(current_amount * stack[0], target, &stack[1..]) {
        return true;
    }

    if is_calculation_possible_v2(concatenate_numbers(current_amount, stack[0]), target, &stack[1..]) {
        return true;
    }

    false
}


pub fn first_part() {
    let start = Instant::now();
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (left.parse::<usize>().unwrap(), right.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect::<Vec<_>>())
        })
        .filter_map(|(test_value, values)| {
            if is_calculation_possible(0, test_value, values.as_slice()) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum::<usize>();
        // .collect::<Vec<_>>();
    println!("{:?}", start.elapsed());

    // 3245122495150
    println!("{:?}", input);
}

pub fn second_part() {
    let start = Instant::now();
    let input = include_str!("input.txt")
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(": ").unwrap();
            (left.parse::<usize>().unwrap(), right.split_whitespace().map(|number| number.parse::<usize>().unwrap()).collect::<Vec<_>>())
        })
        .filter_map(|(test_value, values)| {
            if is_calculation_possible_v2(0, test_value, values.as_slice()) {
                Some(test_value)
            } else {
                None
            }
        })
        .sum::<usize>();
    // .collect::<Vec<_>>();

    println!("{:?}", start.elapsed());
    // 105517128211543
    println!("{:?}", input);

}

fn concatenate_numbers(a: usize, b: usize) -> usize {
    let mut multiplier = 1;
    let mut temp = b;

    // Determine the number of digits in b
    while temp > 0 {
        multiplier *= 10;
        temp /= 10;
    }

    // Concatenate by multiplying a and adding b
    a * multiplier + b
}