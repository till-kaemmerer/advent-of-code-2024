use std::collections::HashMap;

use crate::utils;

pub fn one() {
    print!("===================================\n");
    print!("======= Day 1 - Challenge 1 =======\n");
    print!("===================================\n");

    let (mut a, mut b) = utils::input::get_input_numbers("res/d1/input.txt");
    a.sort();
    b.sort();
    let mut sum = -1;

    for i in 0..a.len() {
        sum += (a[i] - b[i]).abs();
    }

    println!("Solution: {}", sum);
}

pub fn two() {
    print!("===================================\n");
    print!("======= Day 1 - Challenge 2 =======\n");
    print!("===================================\n");

    let (a, b) = utils::input::get_input_numbers("res/d1/input.txt");
    let mut sum = 0;

    let mut occ: HashMap<i32, i32> = HashMap::new();

    for i in 0..b.len() {
        if let Some(x) = occ.get(&b[i]) {
            occ.insert(b[i], x + 1);
        } else {
            occ.insert(b[i], 1);
        }
    }

    for i in 0..a.len() {
        if let Some(x) = occ.get(&a[i]) {
            sum += a[i] * x;
        }
    }

    println!("Solution: {}", sum);
}
