use std::cmp;
use std::error;
use std::fs;
use std::io;
use std::io::prelude::*;

pub fn fuel_needed(mass: i64) -> i64 {
    (mass / 3) - 2
}

pub fn day_1(fuel_calc: &dyn Fn(i64) -> i64) -> Result<(), Box<error::Error>> {
    let file = fs::File::open("data/input_1.txt")?;
    let reader = io::BufReader::new(file);

    let mut total = 0;
    for module in reader.lines() {
        let mass: i64 = module?.parse()?;
        total += fuel_calc(mass)
    }
    println!("{}", total);
    Ok(())
}

#[test]
fn day_1_1_test() {
    assert_eq!(fuel_needed(100756), 33583);
    assert_eq!(fuel_needed(1969), 654);
}

pub fn fuel_needed_compounded(mass: i64) -> i64 {
    let mut cur_portion = mass;
    let mut total = 0;
    
    while cur_portion != 0 {
        cur_portion = cmp::max(0, fuel_needed(cur_portion));
        total += cur_portion;
    }
    total
}

#[test]
fn day_1_2_test() {
    assert_eq!(fuel_needed_compounded(100756), 50346);
    assert_eq!(fuel_needed_compounded(1969), 966);
}
