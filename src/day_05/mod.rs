use std::cmp::max;
use std::collections::HashMap;

pub fn first_part() {
    let mut input = include_str!("input.txt").lines();

    let mut line = input.next();
    let mut order = vec![];
    while matches!(line, Some(contents) if contents != "") {
        let (l, r) = line.unwrap().rsplit_once('|').unwrap();
        order.push((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));
        line = input.next();
    }

    line = input.next();
    let mut updates = vec![];
    while matches!(line, Some(contents) if contents != "") {
        let update = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        updates.push(update);
        line = input.next();
    }

    let mut befores: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut afters: HashMap<usize, Vec<usize>> = HashMap::new();

    order.iter().for_each(|&(l, r)| {
        match afters.get_mut(&l) {
            Some(vec) => {
                vec.push(r);
            },
            None => {
                afters.insert(l, vec![r]);
            }
        }

        match befores.get_mut(&r) {
            Some(vec) => {
                vec.push(l);
            },
            None => {
                befores.insert(r, vec![l]);
            }
        }
    });

    let sum = updates
        .iter()
        .filter_map(|page_numbers| {
            let max = page_numbers.len();
            let mut l_p = 0usize;
            // println!("{:?}", page_numbers);
            // println!("{}", &page_numbers[0]);
            // println!("befores: {:?}", befores.get(&page_numbers[0]));
            // println!("afters: {:?}", afters.get(&page_numbers[0]));
            while l_p < max {
                let mut pointer = l_p + 1;
                while pointer < max {
                    let number = &page_numbers[l_p];
                    let rules = befores.get(number);
                    if let Some(rules) = rules {
                        // println!("{}:{:?}", &page_numbers[l_p], rules);
                        if rules.contains(&page_numbers[pointer]) {
                            // println!("[invalid] {} comes after {}", page_numbers.get(pointer).unwrap(), page_numbers.get(l_p).unwrap());
                            return None;
                        }
                    }
                    pointer += 1;
                }


                let mut pointer: isize = l_p as isize - 1;
                while pointer >= 0 {
                    let number = &page_numbers[l_p];
                    let rules = afters.get(number);
                    if let Some(rules) = rules {
                        if rules.contains(&page_numbers[pointer as usize]) {
                            println!("[invalid] {} comes before {}", page_numbers.get(pointer as usize).unwrap(), page_numbers.get(l_p).unwrap());
                            return None;
                        }
                    }
                    pointer -= 1;
                }

                l_p += 1;
            }

            Some(page_numbers[max / 2])
        })
        .sum::<usize>();

    println!("{}", sum);

    // println!("{:?}", befores);
    // println!("{:?}", afters);
    //
    // println!("{:?}", order);
    // println!("{:?}", updates);
}

pub fn second_part() {
    let mut input = include_str!("input.txt").lines();

    let mut line = input.next();
    let mut order = vec![];
    while matches!(line, Some(contents) if contents != "") {
        let (l, r) = line.unwrap().rsplit_once('|').unwrap();
        order.push((l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap()));
        line = input.next();
    }

    line = input.next();
    let mut updates = vec![];
    while matches!(line, Some(contents) if contents != "") {
        let update = line
            .unwrap()
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        updates.push(update);
        line = input.next();
    }

    let mut befores: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut afters: HashMap<usize, Vec<usize>> = HashMap::new();

    order.iter().for_each(|&(l, r)| {
        match afters.get_mut(&l) {
            Some(vec) => {
                vec.push(r);
            },
            None => {
                afters.insert(l, vec![r]);
            }
        }

        match befores.get_mut(&r) {
            Some(vec) => {
                vec.push(l);
            },
            None => {
                befores.insert(r, vec![l]);
            }
        }
    });

    let sum = updates
        .iter_mut()
        .filter(|page_numbers| {
            let max = page_numbers.len();
            let mut l_p = 0usize;
            // println!("{:?}", page_numbers);
            // println!("{}", &page_numbers[0]);
            // println!("befores: {:?}", befores.get(&page_numbers[0]));
            // println!("afters: {:?}", afters.get(&page_numbers[0]));
            while l_p < max {
                let mut pointer = l_p + 1;
                while pointer < max {
                    let number = &page_numbers[l_p];
                    let rules = befores.get(number);
                    if let Some(rules) = rules {
                        // println!("{}:{:?}", &page_numbers[l_p], rules);
                        if rules.contains(&page_numbers[pointer]) {
                            // println!("[invalid] {} comes after {}", page_numbers.get(pointer).unwrap(), page_numbers.get(l_p).unwrap());
                            return true;
                        }
                    }
                    pointer += 1;
                }


                let mut pointer: isize = l_p as isize - 1;
                while pointer >= 0 {
                    let number = &page_numbers[l_p];
                    let rules = afters.get(number);
                    if let Some(rules) = rules {
                        if rules.contains(&page_numbers[pointer as usize]) {
                            println!("[invalid] {} comes before {}", page_numbers.get(pointer as usize).unwrap(), page_numbers.get(l_p).unwrap());
                            return true
                        }
                    }
                    pointer -= 1;
                }

                l_p += 1;
            }

            false
        })
        .map(|page_numbers| {
            let max = page_numbers.len();

            // let x = sort_updates_in_place(page_numbers, &befores, &afters);
            // println!("{:?}", x);
            println!("{:?}", page_numbers);
            sort_updates_in_place(page_numbers, &befores, &afters);
            println!("{:?}", page_numbers);
            page_numbers[max / 2]
        })
        .sum::<usize>();

    println!("{}", sum);

    // 5367 too high
    // 5402
    // 5507 too high
    // 5110 incorrect
}


use std::collections::{HashSet, VecDeque};

fn sort_updates_in_place(
    updates: &mut Vec<usize>,
    befores: &HashMap<usize, Vec<usize>>,
    afters: &HashMap<usize, Vec<usize>>,
) {
    let max = updates.len();

    'main_loop: loop {
        // correct list
        let mut l_p = 0;
        while l_p < max {
            let rules = befores.get(&updates[l_p]);
            if let Some(rules) = rules {
                // look if the rule applies
                let mut r_p = l_p + 1;
                while r_p < max {
                    if rules.contains(&updates[r_p]) {
                        // rule applies, swap values
                        let temp = updates[r_p];
                        updates[r_p] = updates[l_p];
                        updates[l_p] = temp;
                    }
                    r_p += 1;
                }
                // println!("{}:{:?}", &page_numbers[l_p], rules);

            }
            l_p += 1;
        }

        let mut l_p = max as isize - 1;
        while l_p >= 0 {
            let rules = afters.get(&updates[l_p as usize]);
            if let Some(rules) = rules {
                // look if the rule applies
                let mut r_p = l_p - 1;
                while r_p >= 0 {
                    if rules.contains(&updates[r_p as usize]) {
                        // rule applies, swap values
                        let temp = updates[r_p as usize];
                        updates[r_p as usize] = updates[l_p as usize];
                        updates[l_p as usize] = temp;
                    }
                    r_p -= 1;
                }
                // println!("{}:{:?}", &page_numbers[l_p], rules);

            }
            l_p -= 1;
        }

        // check list
        let mut l_p = 0usize;
        // println!("{:?}", page_numbers);
        // println!("{}", &page_numbers[0]);
        // println!("befores: {:?}", befores.get(&page_numbers[0]));
        // println!("afters: {:?}", afters.get(&page_numbers[0]));
        while l_p < max {
            let mut pointer = l_p + 1;
            while pointer < max {
                let number = &updates[l_p];
                let rules = befores.get(number);
                if let Some(rules) = rules {
                    // println!("{}:{:?}", &page_numbers[l_p], rules);
                    if rules.contains(&updates[pointer]) {
                        // println!("[invalid] {} comes after {}", page_numbers.get(pointer).unwrap(), page_numbers.get(l_p).unwrap());
                        continue 'main_loop;
                    }
                }
                pointer += 1;
            }


            let mut pointer: isize = l_p as isize - 1;
            while pointer >= 0 {
                let number = &updates[l_p];
                let rules = afters.get(number);
                if let Some(rules) = rules {
                    if rules.contains(&updates[pointer as usize]) {
                        println!("[invalid] {} comes before {}", updates.get(pointer as usize).unwrap(), updates.get(l_p).unwrap());
                        continue 'main_loop;
                    }
                }
                pointer -= 1;
            }

            l_p += 1;
        }
    }

}
