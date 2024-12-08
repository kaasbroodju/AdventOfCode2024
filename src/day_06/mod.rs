use std::fmt::{Display, Formatter, Write};

pub fn first_part() {
    let mut input = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| GridElement::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let height = (input.len() - 1) as isize;
    let width = (input[0].len() - 1) as isize;


    let (y, y_vec) = input.iter().enumerate().find(|(_, vec)| vec.contains(&GridElement::Guard)).unwrap();
    let x = y_vec.iter().enumerate().find(|(_, element)| **element == GridElement::Guard).unwrap().0;
    let mut y: isize = y as isize;
    let mut x: isize = x as isize;

    let mut direction = Direction::Up;

    loop {
        // println!("({}, {}): {:?}: {}", x, y, direction, input[y as usize][x as usize]);
        println!("({}, {}): {:?}", x, y, direction);

        match direction {
            Direction::Left => {
                if x > width || x < 0 {break}
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Up;
                    x += 1;
                    continue
                }
                // mark as visited
                input[y as usize][x as usize] = GridElement::Visited;
                // move up
                x -= 1;
            }
            Direction::Right => {
                if x > width || x < 0 {break}
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Down;
                    x -= 1;
                    continue
                }
                // mark as visited
                input[y as usize][x as usize] = GridElement::Visited;
                // move up
                x += 1;
            }
            Direction::Up => {
                if y > height || y < 0 {break}
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Right;
                    y += 1;
                    continue
                }
                // mark as visited
                input[y as usize][x as usize] = GridElement::Visited;
                // move up
                y -= 1;
            }
            Direction::Down => {
                if y > height || y < 0 {break}
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Left;
                    y -= 1;
                    continue
                }
                // mark as visited
                input[y as usize][x as usize] = GridElement::Visited;
                // move up
                y += 1;
            }
        }
    }

    let mut total = 0usize;
    for line in input {
        for x in line {
            if x == GridElement::Visited { total += 1; }
            print!("{}", x);
        }
        println!();
    }


    // let total = input.iter().map(|vec| vec.iter().filter(|element| **element == GridElement::Visited).count()).sum::<usize>();
    println!("Total: {}", total);
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
enum GridElement {
    Empty,
    Guard,
    Visited,
    VisitedV2 {
        directions: Vec<Direction>,
    },
    Obstacle,
}

impl Clone for GridElement {
    fn clone(&self) -> Self {
        match &self {
            GridElement::Empty => { GridElement::Empty }
            GridElement::Guard => { GridElement::Guard }
            GridElement::Visited => { GridElement::Visited }
            GridElement::VisitedV2{ directions } => {
                GridElement::VisitedV2 {
                    directions: directions.clone()
                }
            }
            GridElement::Obstacle => { GridElement::Obstacle }
        }
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl From<char> for GridElement {
    fn from(value: char) -> Self {
        match value {
            '.' => GridElement::Empty,
            '^' => GridElement::Guard,
            'X' => GridElement::Visited,
            '#' => GridElement::Obstacle,
            _ => panic!("Invalid grid element type: {}", value),
        }
    }
}

impl Into<char> for GridElement {
    fn into(self) -> char {
        match self {
            GridElement::Empty => { '.' }
            GridElement::Guard => { '^'}
            GridElement::Visited => { 'X' }
            GridElement::VisitedV2 {directions} => {
                if directions.len() > 1 {
                    '+'
                } else {
                    match directions[0] {
                        Direction::Left | Direction::Right => { '-' }
                        Direction::Down | Direction::Up => { '|' }
                    }
                }
            }
            GridElement::Obstacle => { '#' }
        }
    }
}


impl Display for GridElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(match self {
            GridElement::Empty => { '.' }
            GridElement::Guard => { '^'}
            GridElement::Visited => { 'X' }
            GridElement::VisitedV2 {directions} => {
                if directions.len() > 1 {
                    '+'
                } else {
                    match directions[0] {
                        Direction::Left | Direction::Right => { '-' }
                        Direction::Down | Direction::Up => { '|' }
                    }
                }
            }
            GridElement::Obstacle => { '#' }
        })
    }
}

pub fn second_part() {
    // run simulation every possible grid () coordinate except when there is already obstacle there
    // if the guard has the same direction as a already visited space break, and add to counter he is in a loop
    // if the guard exits the grid stop simulation a useal and go to next step



    let mut input = include_str!("input.txt")
        .lines()
        .map(|line| line.chars().map(|c| GridElement::from(c)).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut possibilities = vec![];
    input.iter().enumerate().for_each(|(y, vec)| vec.iter().enumerate().for_each(|(x, element)| {
        if element == &GridElement::Empty {
            possibilities.push((x, y));
        }
    }));

    let height = (input.len() - 1) as isize;
    let width = (input[0].len() - 1) as isize;


    let (start_pos_y, y_vec) = input.iter().enumerate().find(|(_, vec)| vec.contains(&GridElement::Guard)).unwrap();
    let start_pos_x = y_vec.iter().enumerate().find(|(_, element)| **element == GridElement::Guard).unwrap().0;

    let mut total = 0usize;
    let max = possibilities.len() as f64;
    let mut index = 1f64;
    for (place_object_on_x, place_object_on_y) in possibilities {
        if run_simulation(&mut input.clone(), height, width, start_pos_x, start_pos_y, place_object_on_x, place_object_on_y) {
            println!("loop on: ({}, {})", place_object_on_x, place_object_on_y);
            total += 1;
        }
        println!("{}", index / max * 100.0);
        index += 1.0;
    }


    // let total = input.iter().map(|vec| vec.iter().filter(|element| **element == GridElement::Visited).count()).sum::<usize>();
    println!("Total: {}", total);

}


// return true when guard is in a loop
fn run_simulation(input: &mut Vec<Vec<GridElement>>, height: isize, width: isize, start_pos_x: usize, start_pos_y: usize, place_object_on_x: usize, place_object_on_y: usize) -> bool {
    // let mut grid = &mut *input.clone();
    input[place_object_on_y][place_object_on_x] = GridElement::Obstacle;
    let mut y: isize = start_pos_y as isize;
    let mut x: isize = start_pos_x as isize;
    let mut direction = Direction::Up;
    loop {
        // println!("({}, {}): {:?}: {}", x, y, direction, input[y as usize][x as usize]);
        // println!("({}, {}): {:?}", x, y, direction);

        match direction {
            Direction::Left => {
                if x > width || x < 0 { break }
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Up;
                    x += 1;
                    continue
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { directions }  if directions.contains(&direction)) {
                    // already visited and with same direction, guard is in a loop
                    return true;
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { .. }) {
                    // already visited but not in same direction
                    match &mut input[y as usize][x as usize] {
                        GridElement::VisitedV2 { directions } => {
                            directions.push(Direction::Left);
                        }
                        _ => panic!("Invalid grid element type: {}", input[y as usize][x as usize])
                    }
                } else {
                    // never visited, mark as visited with current direction
                    input[y as usize][x as usize] = GridElement::VisitedV2 {directions: vec![Direction::Left]};
                }
                // move up
                x -= 1;
            }
            Direction::Right => {
                if x > width || x < 0 { break }
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Down;
                    x -= 1;
                    continue
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { directions }  if directions.contains(&direction)) {
                    // already visited and with same direction, guard is in a loop
                    return true;
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { .. }) {
                    // already visited but not in same direction
                    match &mut input[y as usize][x as usize] {
                        GridElement::VisitedV2 { directions } => {
                            directions.push(Direction::Right);
                        }
                        _ => panic!("Invalid grid element type: {}", input[y as usize][x as usize])
                    }
                } else {
                    // never visited, mark as visited with current direction
                    input[y as usize][x as usize] = GridElement::VisitedV2 {directions: vec![Direction::Right]};
                }
                // move up
                x += 1;
            }
            Direction::Up => {
                if y > height || y < 0 { break }
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Right;
                    y += 1;
                    continue
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { directions }  if directions.contains(&direction)) {
                    // already visited and with same direction, guard is in a loop
                    return true;
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { .. }) {
                    // already visited but not in same direction
                    match &mut input[y as usize][x as usize] {
                        GridElement::VisitedV2 { directions } => {
                            directions.push(Direction::Up);
                        }
                        _ => panic!("Invalid grid element type: {}", input[y as usize][x as usize])
                    }
                } else {
                    // never visited, mark as visited with current direction
                    input[y as usize][x as usize] = GridElement::VisitedV2 {directions: vec![Direction::Up]};
                }
                // move up
                y -= 1;
            }
            Direction::Down => {
                if y > height || y < 0 { break }
                if input[y as usize][x as usize] == GridElement::Obstacle {
                    direction = Direction::Left;
                    y -= 1;
                    continue
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { directions }  if directions.contains(&direction)) {
                    // already visited and with same direction, guard is in a loop
                    return true;
                } else if matches!(&input[y as usize][x as usize], GridElement::VisitedV2 { .. }) {
                    // already visited but not in same direction
                    match &mut input[y as usize][x as usize] {
                        GridElement::VisitedV2 { directions } => {
                            directions.push(Direction::Down);
                        }
                        _ => panic!("Invalid grid element type: {}", input[y as usize][x as usize])
                    }
                } else {
                    // never visited, mark as visited with current direction
                    input[y as usize][x as usize] = GridElement::VisitedV2 {directions: vec![Direction::Down]};
                }
                // move up
                y += 1;
            }
        }
    }

    false
}