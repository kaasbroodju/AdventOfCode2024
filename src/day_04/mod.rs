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

    // grid.iter().for_each(|row| println!("{:?}", row));

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

pub fn second_part_one_line() {
    let grid = include_str!("input.txt").as_bytes();

    let width = 140;
    let length = grid.len();

    let next_row = (width + 1) * 2;
    let top_left = &grid[0..length - (next_row) - 2];
    let top_right = &grid[2..length - (next_row)];
    let start_index_middle = width + 1 + 1;
    let end_index_middle = length - (width + 1) - 1;
    let middle = &grid[start_index_middle..end_index_middle];
    let bottom_left = &grid[next_row..length - 2];
    let bottom_right = &grid[(next_row) + 2..length];

    let mut i = 0;
    let length = end_index_middle - start_index_middle;
    let mut count = 0usize;
    while i < length {
        count += ((middle[i] == b'A')
            // /
            && ((bottom_left[i] == b'M' && top_right[i] == b'S') || (bottom_left[i] == b'S' && top_right[i] == b'M'))
            // \
            && ((top_left[i] == b'M' && bottom_right[i] == b'S') || (top_left[i] == b'S' && bottom_right[i] == b'M'))) as usize;
        i += 1;
    }

    println!("{}", count);
}


use std::simd::{Simd};
use std::simd::prelude::SimdPartialEq;

pub fn second_part_one_line_simd() {
    let grid = include_str!("input.txt").as_bytes();

    let width = 140;
    let length = grid.len();

    let next_row = (width + 1) * 2;
    let top_left = &grid[0..length - next_row - 2];
    let top_right = &grid[2..length - next_row];
    let start_index_middle = width + 2;
    let end_index_middle = length - (width + 1) - 1;
    let middle = &grid[start_index_middle..end_index_middle];
    let bottom_left = &grid[next_row..length - 2];
    let bottom_right = &grid[next_row + 2..length];

    let mut count = 0usize;

    // let chunk_size = Simd::<u8, 64>::LEN; // Use 64 lanes for optimal performance on modern architectures
    let chunk_size = Simd::<u8, 64>::LEN; // Use 64 lanes for optimal performance on modern architectures
    let length = end_index_middle - start_index_middle;

    let mut i = 0;
    while i + chunk_size <= length {
        // Load slices into SIMD vectors
        let middle_vec: Simd<u8, 64> = Simd::from_slice(&middle[i..]);
        let bottom_left_vec = Simd::from_slice(&bottom_left[i..]);
        let top_right_vec = Simd::from_slice(&top_right[i..]);
        let top_left_vec = Simd::from_slice(&top_left[i..]);
        let bottom_right_vec = Simd::from_slice(&bottom_right[i..]);

        // Compare middle bytes with 'A'
        let middle_a_mask = middle_vec.simd_eq(Simd::splat(b'A'));

        // Check diagonal conditions for /
        let bl_m_tr_s_mask = bottom_left_vec.simd_eq(Simd::splat(b'M'))
            & top_right_vec.simd_eq(Simd::splat(b'S'));
        let bl_s_tr_m_mask = bottom_left_vec.simd_eq(Simd::splat(b'S'))
            & top_right_vec.simd_eq(Simd::splat(b'M'));
        let diagonal1_mask = bl_m_tr_s_mask | bl_s_tr_m_mask;

        // Check diagonal conditions for \
        let tl_m_br_s_mask = top_left_vec.simd_eq(Simd::splat(b'M'))
            & bottom_right_vec.simd_eq(Simd::splat(b'S'));
        let tl_s_br_m_mask = top_left_vec.simd_eq(Simd::splat(b'S'))
            & bottom_right_vec.simd_eq(Simd::splat(b'M'));
        let diagonal2_mask = tl_m_br_s_mask | tl_s_br_m_mask;

        // Combine all conditions
        let result_mask = middle_a_mask & diagonal1_mask & diagonal2_mask;

        // Count matches
        count += result_mask.to_array().iter().filter(|&&x| x).count();

        i += chunk_size;
    }

    // Process remaining elements with scalar fallback
    while i < length {
        count += ((middle[i] == b'A')
            && ((bottom_left[i] == b'M' && top_right[i] == b'S')
            || (bottom_left[i] == b'S' && top_right[i] == b'M'))
            && ((top_left[i] == b'M' && bottom_right[i] == b'S')
            || (top_left[i] == b'S' && bottom_right[i] == b'M'))) as usize;
        i += 1;
    }

    println!("{}", count);
}
