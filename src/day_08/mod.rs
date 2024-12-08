const bucket_width: usize = (b'z' - b'0') as usize + 1usize;

pub fn first_part() {
    let mut bucket: [Vec<(isize, isize)>; bucket_width] = std::array::from_fn(|_| vec![]);
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != b'.' {
                bucket[(c - b'0') as usize].push((x as isize, y as isize));
            }
        }
    }

    let counter = bucket
        .iter()
        .fold(vec![], |mut acc, elements| {
            let mut i = 0usize;
            let mut j = 0usize;
            let max = elements.len();
            if max == 0 { return acc; }
            while i < max - 1 {
                j = i + 1;
                while j < max {
                    acc.push((elements[i], elements[j]));
                    j += 1;
                }
                i += 1;
            }

            acc
        })
        .iter()
        .fold(vec![], |mut acc, &((l_x, l_y), (r_x, r_y))| {
            let delta_x = r_x - l_x;
            let delta_y = r_y - l_y;

            acc.push((l_x - delta_x, l_y - delta_y));
            acc.push((r_x + delta_x, r_y + delta_y));

            acc
        })
        .iter()
        .filter(|&&(x, y)| (x >= 0 && x < width as isize) && (y >= 0 && y < height as isize))
        .fold(vec![0u64; height], |mut acc, &(x, y)| {
            acc[y as usize] |= 1 << x;
            acc
        })
        .iter()
        .map(|&x| x.count_ones())
        .sum::<u32>();

    // 409
    println!("{}", counter);
}

pub fn second_part() {
    let mut bucket: [Vec<(isize, isize)>; bucket_width] = std::array::from_fn(|_| vec![]);
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();

    for (y, line) in grid.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c != b'.' {
                bucket[(c - b'0') as usize].push((x as isize, y as isize));
            }
        }
    }

    let counter = bucket
        .iter()
        .fold(vec![], |mut acc, elements| {
            let mut i = 0usize;
            let mut j = 0usize;
            let max = elements.len();
            if max == 0 { return acc; }
            while i < max - 1 {
                j = i + 1;
                while j < max {
                    acc.push((elements[i], elements[j]));
                    j += 1;
                }
                i += 1;
            }

            acc
        })
        .iter()
        .fold(vec![], |mut points, &((l_x, l_y), (r_x, r_y))| {
            let delta_x = r_x - l_x;
            let delta_y = r_y - l_y;

            let mut l_x = l_x - delta_x;
            let mut l_y = l_y - delta_y;
            while (l_x >= 0 && l_x < width as isize) && (l_y >= 0 && l_y < height as isize) {
                points.push((l_x, l_y));
                l_x -= delta_x;
                l_y -= delta_y;
            }

            let mut r_x = r_x - delta_x;
            let mut r_y = r_y - delta_y;
            while (r_x >= 0 && r_x < width as isize) && (r_y >= 0 && r_y < height as isize) {
                points.push((r_x, r_y));
                r_x += delta_x;
                r_y += delta_y;
            }

            points
        })
        .iter()
        .fold(vec![0u64; height], |mut acc, &(x, y)| {
            acc[y as usize] |= 1 << x;
            acc
        })
        .iter()
        .map(|&x| x.count_ones())
        .sum::<u32>();

    // 1308
    println!("{}", counter);
}