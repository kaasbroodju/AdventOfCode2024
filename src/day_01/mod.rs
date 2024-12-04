use std::ops::Sub;

pub fn first_part() {
    let mut left_side: Vec<usize> = vec![];
    let mut right_side: Vec<usize> = vec![];

    include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .for_each(|line| {
            let left_number = (line[0] - b'0') as usize * 10000 + (line[1] - b'0') as usize * 1000 + (line[2] - b'0') as usize * 100 + (line[3] - b'0') as usize * 10 + (line[4] - b'0') as usize;
            let idx = left_side.partition_point(|&x| x <= left_number);
            left_side.insert(idx, left_number);

            let right_number = (line[8] - b'0') as usize * 10000 + (line[9] - b'0') as usize * 1000 + (line[10] - b'0') as usize * 100 + (line[11] - b'0') as usize * 10 + (line[12] - b'0') as usize;
            let idx = right_side.partition_point(|&x| x <= right_number);
            right_side.insert(idx, right_number);
        });

    let delta = left_side
        .iter()
        .zip(right_side.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum::<usize>();

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
            delta_score.push((l * counter));
        }
    }

    println!("part one: {}", delta);
    println!("part two: {:?}", delta_score);
    println!("part two: {:?}", delta_score.iter().sum::<usize>());

    // println!("{:?}", left_side);
    // println!("{:?}", right_side)
}
