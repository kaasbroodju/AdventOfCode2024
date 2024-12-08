use std::time::Instant;

// todo rewrite to (binary) tree
// recursive
fn idk(current_amount: usize, target: usize, stack: &[usize]) -> bool {
    let is_empty = stack.len() == 0;
    if current_amount == target && is_empty { return true; }
    if is_empty { return false; }
    if idk(current_amount + stack[0], target, &stack[1..]) {
        return true;
    }

    if idk(current_amount * stack[0], target, &stack[1..]) {
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
            return if idk(0, test_value, &values[..]) {
                Some(test_value)
            } else {
                None
            };


            let possibilities = generate_combinations(values.len());

            // println!("{:?}", possibilities);
            for possibility in possibilities {
                let mut total = values[0];
                for (index, operation) in possibility.iter().enumerate() {
                    match operation {
                        Operation::Add => {
                            total += values[index + 1];
                        }
                        Operation::Multiply => {
                            total *= values[index + 1];
                        }
                    }
                    if total == test_value {
                        return Some(test_value);
                    }
                }
            }

            None
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
            let possibilities = generate_combinationsV2(values.len());

            // println!("{:?}", possibilities);
            for possibility in possibilities {
                let mut total = values[0];
                for (index, operation) in possibility.iter().enumerate() {
                    match operation {
                        OperationV2::Add => {
                            total += values[index + 1];
                        }
                        OperationV2::Multiply => {
                            total *= values[index + 1];
                        }
                        OperationV2::Concat => {
                            total = concatenate_numbers(total, values[index + 1])
                        }
                    }
                }
                if total == test_value {
                    return Some(test_value);
                }
            }

            None
        })
        .sum::<usize>();
    // .collect::<Vec<_>>();

    println!("{:?}", start.elapsed());
    // 105517128211543
    println!("{:?}", input);

}

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Multiply,
}

// Generate all possible combinations of n-1 items.
fn generate_combinations(n: usize) -> Vec<Vec<Operation>> {
    if n == 0 {
        return vec![];
    }

    // Start with a single element.
    let mut combinations = vec![vec![Operation::Add], vec![Operation::Multiply]];

    // Add elements to reach n-1.
    for _ in 1..(n - 1) {
        let mut new_combinations = Vec::new();
        for combination in &combinations {
            let mut with_a = combination.clone();
            with_a.push(Operation::Add);

            let mut with_b = combination.clone();
            with_b.push(Operation::Multiply);

            new_combinations.push(with_a);
            new_combinations.push(with_b);
        }
        combinations = new_combinations
    }

    combinations
}

#[derive(Debug, Clone, Copy)]
enum OperationV2 {
    Add,
    Multiply,
    Concat,
}

// Generate all possible combinations of n-1 items.
fn generate_combinationsV2(n: usize) -> Vec<Vec<OperationV2>> {
    if n == 0 {
        return vec![];
    }

    // Start with a single element.
    let mut combinations = vec![
        vec![OperationV2::Add],
        vec![OperationV2::Multiply],
        vec![OperationV2::Concat],
    ];

    // Add elements to reach n-1.
    for _ in 1..(n - 1) {
        let mut new_combinations = Vec::new();
        for combination in &combinations {
            let mut with_a = combination.clone();
            with_a.push(OperationV2::Add);

            let mut with_b = combination.clone();
            with_b.push(OperationV2::Multiply);

            let mut with_c = combination.clone();
            with_c.push(OperationV2::Concat);

            new_combinations.push(with_a);
            new_combinations.push(with_b);
            new_combinations.push(with_c);
        }
        combinations = new_combinations;
    }

    combinations
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