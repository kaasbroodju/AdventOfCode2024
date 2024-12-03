pub fn first_part() {
    let reports = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(|number| number.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();


    let mut amount_safe = 0;
    'report_loop: for levels in reports {
        let delta = levels[1] - levels[0];
        // println!("{}", delta);
        if delta == 0 || delta.abs() > 3 { continue; }
        let is_going_upwards = delta > 0;
        let max_index = levels.len() - 1;
        let mut index = 0;
        while index < max_index {
            let delta = levels[index + 1] - levels[index];
            if delta == 0 || delta.abs() > 3 || (is_going_upwards != (delta > 0)) { continue 'report_loop; }
            index += 1;
        }
        // println!("safe: {:?}", levels);
        amount_safe += 1;
    }

    println!("{}", amount_safe);
}

pub fn second_part() {
    let mut reports = include_str!("input.txt")
        .lines()
        .map(|line| line.split_whitespace().map(|number| number.parse::<isize>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();


    let mut unsafe_reports = vec![];
    let mut amount_safe = 0;
    'report_loop: for levels in reports {
        let delta = levels[1] - levels[0];
        // println!("{}", delta);
        if delta == 0 || delta.abs() > 3 { continue; }
        let is_going_upwards = delta > 0;
        let max_index = levels.len() - 1;
        let mut index = 0;
        while index < max_index {
            let delta = levels[index + 1] - levels[index];
            if delta == 0 || delta.abs() > 3 || (is_going_upwards != (delta > 0)) {
                unsafe_reports.push(levels);
                continue 'report_loop;
            }
            index += 1;
        }
        // println!("safe: {:?}", levels);
        amount_safe += 1;
    }

    let mut still_safe = 0;
    'report_loop: for levels in unsafe_reports {
        let delta = levels[1] - levels[0];
        // println!("{}", delta);
        if delta == 0 || delta.abs() > 3 { continue; }
        let is_going_upwards = delta > 0;
        let max_index = levels.len() - 1;
        let mut index = 0;
        while index < max_index {
            let delta = levels[index + 1] - levels[index];
            if delta == 0 || delta.abs() > 3 || (is_going_upwards != (delta > 0)) {
                continue 'report_loop;
            }
            index += 1;
        }
        // println!("safe: {:?}", levels);
        still_safe += 1;
    }

    println!("{}", amount_safe);

    // 552 x
    // 589
    // 565
    // 564 x
    // 558
}