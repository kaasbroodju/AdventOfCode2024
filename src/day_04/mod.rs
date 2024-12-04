pub fn first_part() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    // grid.iter().for_each(|row| println!("{:?}", row));

    let width = grid[0].len();
    let height = grid.len();

    let mut i = 0;
    let mut j = 0;
    let mut count = 0usize;
    while i < height {
        j = 0;
        while j < width {
            // println!("{}, {}", grid[i][j], grid[i][j + 1]);
            if grid[i][j] == 'X' {
                // println!("{}, {}", grid[i][j], grid[i][j + 1]);
                // forward horizontal
                // ^
                if check_upward(&grid, &i, &j) {count += 1};
                // /
                if check_forward_horizontal_upward(&grid, &i, &j) {count += 1};
                // >
                if check_forward(&grid, &i, &j) {count += 1};
                // \
                if check_forward_horizontal_downward(&grid, &i, &j) {count += 1};
                // v
                if check_downward(&grid, &i, &j) {count += 1};
                // /
                if check_backward_horizontal_downward(&grid, &i, &j) {count += 1};
                // <
                if check_backward(&grid, &i, &j) {count += 1};
                // \
                if check_backward_horizontal_upward(&grid, &i, &j) {count += 1};



            }

            j += 1;
        }

        i += 1;
    }

    // println!("{}", count);
}

fn check_forward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    // println!("i: {:?}, j: {:?}", i, j);
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j+1), Some('M'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j+2), Some('A'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j+3), Some('S'))) {return false}
    // println!("i: {:?}, j: {:?}", i, j);
    true
}

fn check_backward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if *j < 3 {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j-1), Some('M'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j-2), Some('A'))) {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j-3), Some('S'))) {return false}
    true
}

fn check_backward_horizontal_downward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if *j < 3 {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i+1), Some(row) if matches!(row.get(*j-1), Some('M'))) {return false}
    if !matches!(grid.get(*i+2), Some(row) if matches!(row.get(*j-2), Some('A'))) {return false}
    if !matches!(grid.get(*i+3), Some(row) if matches!(row.get(*j-3), Some('S'))) {return false}
    true
}

fn check_backward_horizontal_upward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if *j < 3 || *i < 3 {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i-1), Some(row) if matches!(row.get(*j-1), Some('M'))) {return false}
    if !matches!(grid.get(*i-2), Some(row) if matches!(row.get(*j-2), Some('A'))) {return false}
    if !matches!(grid.get(*i-3), Some(row) if matches!(row.get(*j-3), Some('S'))) {return false}
    true
}

fn check_forward_horizontal_upward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if *i < 3 {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i-1), Some(row) if matches!(row.get(*j+1), Some('M'))) {return false}
    if !matches!(grid.get(*i-2), Some(row) if matches!(row.get(*j+2), Some('A'))) {return false}
    if !matches!(grid.get(*i-3), Some(row) if matches!(row.get(*j+3), Some('S'))) {return false}
    true
}

fn check_forward_horizontal_downward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i+1), Some(row) if matches!(row.get(*j+1), Some('M'))) {return false}
    if !matches!(grid.get(*i+2), Some(row) if matches!(row.get(*j+2), Some('A'))) {return false}
    if !matches!(grid.get(*i+3), Some(row) if matches!(row.get(*j+3), Some('S'))) {return false}
    true
}

fn check_downward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i+1), Some(row) if matches!(row.get(*j), Some('M'))) {return false}
    if !matches!(grid.get(*i+2), Some(row) if matches!(row.get(*j), Some('A'))) {return false}
    if !matches!(grid.get(*i+3), Some(row) if matches!(row.get(*j), Some('S'))) {return false}
    true
}

fn check_upward(grid: &Vec<Vec<char>>, i: &usize, j: &usize) -> bool {
    if *i < 3 {return false}
    if !matches!(grid.get(*i), Some(row) if matches!(row.get(*j), Some('X'))) {return false}
    if !matches!(grid.get(*i-1), Some(row) if matches!(row.get(*j), Some('M'))) {return false}
    if !matches!(grid.get(*i-2), Some(row) if matches!(row.get(*j), Some('A'))) {return false}
    if !matches!(grid.get(*i-3), Some(row) if matches!(row.get(*j), Some('S'))) {return false}
    true
}

pub fn second_part() {
    let grid = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    grid.iter().for_each(|row| println!("{:?}", row));

    let width = grid[0].len() - 1;
    let height = grid.len() - 1;

    let mut i = 1;
    let mut j = 1;
    let mut count = 0usize;
    while i < height {
        j = 1;
        while j < width {
            // println!("{}, {}", grid[i][j], grid[i][j + 1]);
            if grid[i][j] == 'A' {
                // /
                let correct_forward = (grid[i+1][j-1] == 'M' && grid[i-1][j+1] == 'S') || (grid[i+1][j-1] == 'S' && grid[i-1][j+1] == 'M');

                // \
                let correct_backward = (grid[i-1][j-1] == 'M' && grid[i+1][j+1] == 'S') || (grid[i-1][j-1] == 'S' && grid[i+1][j+1] == 'M');

                if correct_forward && correct_backward {
                    count += 1;
                }

            }

            j += 1;
        }

        i += 1;
    }

    println!("{}", count);
}