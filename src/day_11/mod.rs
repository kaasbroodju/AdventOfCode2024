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
    small_cache: [[Option<usize>; ROUNDS]; 10],
    cache: [Vec<(usize, [Option<usize>; ROUNDS])>; AMOUNT_OF_BUCKET],

}

impl<const ROUNDS_BUCKET: usize> PlutoStones<ROUNDS_BUCKET> {
    fn new() -> Self {
        Self {
            small_cache: std::array::from_fn(|_| std::array::from_fn(|_| None)),
            cache: std::array::from_fn(|_| Vec::with_capacity(32)),
        }
    }

    fn get_amount_of_stones(&mut self, number: usize, round: usize) -> usize {
        if round == 0 { return 1; }

        // check single numbers
        if number < 10 {
            if let Some(cached_value) = self.small_cache[number][round] {
                // calculation is already made
                return cached_value;
            } else {
                // Calculate amount of stones recursively
                let amount_of_stones = if number == 0 {
                    self.get_amount_of_stones(1, round - 1)
                } else {
                    self.get_amount_of_stones(number * 2024, round - 1)
                };

                // Set calculation in cache
                self.small_cache[number][round] = Some(amount_of_stones);

                return amount_of_stones;
            }
        }

        let bucket = number % AMOUNT_OF_BUCKET;

        let cached_value = self.cache[bucket]
            .binary_search_by(|(probe, _)| probe.cmp(&number))
            .map(|pre_calculated_round| self.cache[bucket][pre_calculated_round].1[round]);

        match cached_value {
            Ok(potential_cached_value) => {
                if let Some(cached_value) = potential_cached_value {
                    // calculation is already made
                    return cached_value;
                } else {
                    // Calculate amount of stones recursively
                    let amount_of_stones = if has_even_digits(number) {
                        let (left, right) = split_integer(number);
                        self.get_amount_of_stones(left, round - 1) + self.get_amount_of_stones(right, round - 1)
                    } else {
                        self.get_amount_of_stones(number * 2024, round - 1)
                    };

                    let index = self.cache[bucket].binary_search_by(|(probe, _)| probe.cmp(&number)).unwrap();

                    // Set calculation in cache
                    self.cache[bucket][index].1[round] = Some(amount_of_stones);

                    return amount_of_stones;
                }
            }
            Err(index) => {
                // Number is not cached, initialize cache
                self.cache[bucket].insert(index, (number, std::array::from_fn(|_| None)));

                return self.get_amount_of_stones(number, round);
            }
        }
    }
}

fn has_even_digits(number: usize) -> bool {
    let mut count = 0;
    let mut n = number;

    while n > 0 {
        n /= 10; // Remove the last digit
        count += 1; // Count the digits
    }

    count % 2 == 0
}

#[inline]
fn split_integer(number: usize) -> (usize, usize) {
    let num_digits = number.ilog10() as usize + 1;
    // let num_digits = (number as f64).log10().floor() as usize + 1;
    let half = num_digits / 2;

    // Compute the divisor to split the number
    let divisor = 10usize.pow(half as u32);

    let left = number / divisor;      // Left half
    let right = number % divisor;    // Right half

    (left, right)
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
    println!("{:?}", start.elapsed()); // 66sms
    println!("{:?}", total);
    // 229682160383225
}