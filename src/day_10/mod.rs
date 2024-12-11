use std::collections::HashSet;

pub fn first_part() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = input.len();
    let width = input[0].len();

    let zero_locations = input
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (y, line)| {
            for (x, v) in line.iter().enumerate() {
                if *v == 0 {
                    acc.push((x, y))
                }
            }

            acc
        })
        .iter()
        .map(|(x,y)| {
            let mut output = HashSet::new();
            get_trailheads(&input, &(width as isize), &(height as isize), &mut output, (*x as isize, *y as isize), 0);
            output.len()
        })
        .sum::<usize>();
        // .collect::<Vec<_>>();

    println!("{:?}", zero_locations);
    // 644
}

fn get_trailheads(grid: &Vec<Vec<u8>>, width: &isize, height: &isize, output: &mut HashSet<(usize, usize)>, (x, y): (isize, isize), target: usize) {

    if x < 0 || x >= *width { return; }
    if y < 0 || y >= *height { return; }

    let value = grid[y as usize][x as usize];
    // println!("({}, {}): {}|{}", x, y, value, target);

    if value as usize != target { return; }
    if value == 9 && target == 9 {
        // println!("Found trailhead!");
        output.insert((x as usize, y as usize));
    }

    // left
    get_trailheads(grid, width, height, output, (x - 1, y), target + 1);
    // up
    get_trailheads(grid, width, height, output, (x, y - 1), target + 1);
    // right
    get_trailheads(grid, width, height, output, (x + 1, y), target + 1);
    // down
    get_trailheads(grid, width, height, output, (x, y + 1), target + 1);
}

pub fn first_part_bogaloo() {
    // let grid: [Vec<u64>; 9] = [vec![]; 9];
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = input.len();
    let width = input[0].len();

    let mut buf = vec![0; 10];
    buf.fill(0u64);
    let grid: [Vec<u64>; 9] = std::array::from_fn(|_| buf.clone());


    let zero_locations = input
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (y, line)| {
            for (x, v) in line.iter().enumerate() {
                if *v == 0 {
                    acc.push((x, y))
                }
            }

            acc
        })
        .iter()
        .map(|(x,y)| {
            let mut output = HashSet::new();
            crate::day_10::get_trailheads(&input, &(width as isize), &(height as isize), &mut output, (*x as isize, *y as isize), 0);
            output.len()
        })
        .sum::<usize>();
    // .collect::<Vec<_>>();

    println!("{:?}", zero_locations);
    // 644
}

fn get_trailheads_electric_bogaloo(grid: &Vec<Vec<u8>>, width: &isize, height: &isize, output: &mut HashSet<(usize, usize)>, (x, y): (isize, isize), target: usize) {

    if x < 0 || x >= *width { return; }
    if y < 0 || y >= *height { return; }

    let value = grid[y as usize][x as usize];
    // println!("({}, {}): {}|{}", x, y, value, target);

    if value as usize != target { return; }
    if value == 9 && target == 9 {
        // println!("Found trailhead!");
        output.insert((x as usize, y as usize));
    }

    // left
    get_trailheads(grid, width, height, output, (x - 1, y), target + 1);
    // up
    get_trailheads(grid, width, height, output, (x, y - 1), target + 1);
    // right
    get_trailheads(grid, width, height, output, (x + 1, y), target + 1);
    // down
    get_trailheads(grid, width, height, output, (x, y + 1), target + 1);
}

pub fn second_part() {
    let input = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes().iter().map(|&b| b - b'0').collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = input.len();
    let width = input[0].len();

    let zero_locations = input
        .iter()
        .enumerate()
        .fold(vec![], |mut acc, (y, line)| {
            for (x, v) in line.iter().enumerate() {
                if *v == 0 {
                    acc.push((x, y))
                }
            }

            acc
        })
        .iter()
        .map(|(x,y)| {
            get_trailheads_v2(&input, &(width as isize), &(height as isize), (*x as isize, *y as isize), 0)
        })
        .sum::<usize>();

    println!("{}", zero_locations);

    // 1366
}

fn get_trailheads_v2(grid: &Vec<Vec<u8>>, width: &isize, height: &isize, (x, y): (isize, isize), target: usize) -> usize {

    // out of bounds check
    if x < 0 || x >= *width { return 0; }
    if y < 0 || y >= *height { return 0; }

    let value = grid[y as usize][x as usize];


    if value as usize != target { return 0; }
    if value == 9 && target == 9 {
        // println!("Found trailhead!");
        return 1;
    }

    // left
    get_trailheads_v2(grid, width, height, (x - 1, y), target + 1) +
    // up
    get_trailheads_v2(grid, width, height, (x, y - 1), target + 1) +
    // right
    get_trailheads_v2(grid, width, height, (x + 1, y), target + 1) +
    // down
    get_trailheads_v2(grid, width, height, (x, y + 1), target + 1)
}