use std::collections::{BTreeMap, HashMap};
use std::time::Instant;

pub fn first_part() {
    const ROUNDS: usize = 25;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split_whitespace().map(|number| number.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>()
        .first()
        .unwrap()
        .to_owned();


    let mut grid: PlutoStones<{ ROUNDS + 1 }> = PlutoStones::new();
    let mut total = 0;
    for number in input {
        total += grid.get_amount_of_stones(number, ROUNDS);
    }

    println!("{:?}", total);
    // 193899
}

const AMOUNT_OF_BUCKET: usize = u8::MAX as usize;

#[derive(Debug)]
struct PlutoStones<const ROUNDS: usize> {
    cache: [BTreeMap<usize, [Option<usize>; ROUNDS]>; AMOUNT_OF_BUCKET],
}

impl<const ROUNDS_BUCKET: usize> PlutoStones<ROUNDS_BUCKET> {
    // Constructor for PlutoStonesV2
    fn new() -> Self {
        Self {
            cache: std::array::from_fn(|_| BTreeMap::new()),
        }
    }

    fn get_amount_of_stones(&mut self, number: usize, round: usize) -> usize {
        if round == 0 { return 1; }

        let bucket = number % AMOUNT_OF_BUCKET;

        let cached_value = self.cache[bucket]
            .get(&number)
            .map_or_else(
                || Err(()),
                |pre_calculated_round| Ok(pre_calculated_round[round])
            );

        match cached_value {
            Ok(potential_cached_value) => {
                match potential_cached_value {
                    Some(cached_value) => { cached_value }
                    None => {
                        // Calculate amount of stones when not cached for this round
                        let amount_of_stones = if number == 0 {
                            self.get_amount_of_stones(1, round - 1)
                        } else if number.to_string().len() % 2 == 0 {
                            let string_number = number.to_string();
                            let (left, right) = string_number.split_at(string_number.len() / 2);
                            self.get_amount_of_stones(left.parse().unwrap(), round - 1)
                                + self.get_amount_of_stones(right.parse().unwrap(), round - 1)
                        } else {
                            self.get_amount_of_stones(number * 2024, round - 1)
                        };
                        self.cache[bucket].get_mut(&number).unwrap()[round] = Some(amount_of_stones);
                        amount_of_stones
                    }
                }
            }
            Err(_) => {
                // If the number is not cached, initialize it
                let grid = std::array::from_fn(|_| None);
                self.cache[bucket].insert(number, grid);

                self.get_amount_of_stones(number, round)
            }
        }
    }
}

pub fn second_part() {
    const ROUNDS: usize = 75;

    let input = include_str!("input.txt")
        .lines()
        .map(|l| l.split_whitespace().map(|number| number.parse().unwrap()).collect())
        .collect::<Vec<Vec<usize>>>()
        .first()
        .unwrap()
        .to_owned();


    let start = Instant::now();
    let mut grid: PlutoStones<{ ROUNDS + 1 }> = PlutoStones::new();
    let mut total = 0;
    for number in input {
        total += grid.get_amount_of_stones(number, ROUNDS);
    }
    println!("{:?}", start.elapsed()); // 151ms
    println!("{:?}", total);
    // 229682160383225
}