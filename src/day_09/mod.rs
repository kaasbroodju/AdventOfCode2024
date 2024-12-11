pub fn first_part() {
    let mut input = include_str!("input.txt")
        .as_bytes()
        .iter()
        .map(|&b| b - b'0')
        .enumerate()
        .fold(vec![], |mut acc, (index, value)| {
            if index % 2 == 0 {
                for x in 0..value {
                    // println!("{}: {}", x, index / 2);
                    acc.push(Some(index / 2))
                }

            } else {
                for x in 0..value {
                    // println!("{}: {:?}", x, "None");
                    acc.push(None)
                }
            }
            acc
        });

    let mut l_p = 0;
    let mut slice = &mut input;
    let mut r_p = slice.len() - 1;
    while l_p <= r_p {
        let l_v = slice[l_p];
        let r_v = slice[r_p];
        let l_has_value = l_v.is_some();
        let r_has_value = r_v.is_some();

        match (l_has_value, r_has_value) {
            // '.' '9'
            (false, true) => {
                // swap
                slice[l_p] = slice[r_p];
                slice[r_p] = None;

                l_p += 1;
                r_p -= 1;

            }
            // '9' '9'
            (true, true) => {
                l_p += 1;
            }
            // '.' '.'
            (false, false) => {
                r_p -= 1;
            }
            // '9' '.'
            (true, false) => {
                l_p += 1;
                r_p -= 1;
            }
        }
    }

    println!("{:?}", input);
    let total = input
        .iter()
        .enumerate()
        .map(|(i, v)| {
            match v {
                None => { 0u128 }
                Some(value) => {
                    (i as u128) * (*value as u128)
                }
            }
        })
        .sum::<u128>();

    println!("{}", total);
    // println!("{}", total);

    // 6359213660505
}

#[derive(Clone, Debug)]
enum Fragment {
    File {
        id: usize,
        length: usize,
    },
    Space {
        length: usize,
    },
}

pub fn second_part() {
    let mut input = include_str!("input.txt")
        .as_bytes()
        .iter()
        .map(|&b| b - b'0')
        .enumerate()
        .fold(vec![], |mut acc, (index, value)| {
            if index % 2 == 0 {
                acc.push(Fragment::File { id: index / 2, length: value as usize });
            } else {
                acc.push(Fragment::Space { length: value as usize });
            }
            acc
        });

    let mut l_p = 0;

    let mut r_p = input.len() - 1;

    while r_p != 0 {
        let l_v = &input[l_p];
        let r_v = &input[r_p];

        match (l_v, r_v) {
            (Fragment::Space { length: empty_length }, Fragment::File { length: file_size, id }) => {
                // file fits here
                if file_size < empty_length {
                    let empty = Fragment::Space { length: *file_size };
                    let temp = input[r_p].clone();

                    input[l_p] = Fragment::Space { length: *empty_length - *file_size };
                    input[r_p] = empty;
                    input.insert(l_p, temp);

                    l_p = 0;
                } else if file_size == empty_length {
                    // file fits exact
                    let temp = input[l_p].clone();
                    input[l_p] = input[r_p].clone();
                    input[r_p] = temp;
                    l_p = 0;
                    r_p -= 1;
                } else {
                    l_p += 1;
                }
            }
            (Fragment::File { id: l_id, .. }, Fragment::File { id: r_id, .. }) => {
                if l_id == r_id {
                    l_p = 0;
                    r_p -= 1;
                } else {
                    l_p += 1;
                }
            }
            (Fragment::Space { .. }, Fragment::Space { .. }) => {
                r_p -= 1;
            }
            (Fragment::File { .. }, Fragment::Space { .. }) => {
                r_p -= 1;
            }
        }
    }



    println!("{:?}", input);
    let mut total = 0u128;
    let mut index = 0usize;
    for fragment in input {
        match fragment {
            Fragment::File { length, id } => {
                for _ in 0..length {
                    total += (index as u128) * (id as u128);
                    index += 1;
                }
            }
            Fragment::Space { length } => {
                index += length;
            }
        }
    }

    // let total = input
    //     .iter()
    //     .enumerate()
    //     .map(|(i, v)| {
    //         match v {
    //             None => { 0u128 }
    //             Some(value) => {
    //                 (i as u128) * (*value as u128)
    //             }
    //         }
    //     })
    //     .sum::<u128>();
    //
    println!("{}", total);

    
    // 6381624803796
}