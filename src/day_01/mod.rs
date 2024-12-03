pub fn first_part() {
    let mut left_side = vec![];
    let mut right_side = vec![];

    include_str!("input.txt").lines().for_each(|line| {
        let (l, r) = line.split_once("   ").unwrap();
        left_side.push(l.parse::<i32>().unwrap());
        right_side.push(r.parse::<i32>().unwrap());
    });

    left_side.sort();
    right_side.sort();
    let delta = left_side
        .iter()
        .zip(right_side.iter())
        .map(|(l, r)| (r - l).abs())
        .sum::<i32>();

    let mut delta_score = vec![];
    let mut i = 0;
    for l in left_side {
        let mut counter = 0;
        let index = right_side.iter().position(|r| l == *r);
        if let Some(i) = index {
            let mut i = i;
            while let Some(r) = right_side.get(i) {
                if l != *r { break; }
                counter += 1;
                i += 1;
            }
            delta_score.push((l * counter).abs());
        }
    }

    println!("part one: {}", delta);
    println!("part two: {:?}", delta_score);
    println!("part two: {:?}", delta_score.iter().sum::<i32>());

    // println!("{:?}", left_side);
    // println!("{:?}", right_side)
}
