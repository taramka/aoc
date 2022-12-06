use std::fs;

pub fn main() {
    let contents = fs::read_to_string("inputs/day01.txt")
        .expect("could not read the file");

    let mut cals = Vec::new();
    let mut cur_cals: i32 = 0;

    for item in contents.lines() {
        if item.is_empty() {
            cals.push(cur_cals);
            cur_cals = 0;
        } else {
            cur_cals += item.parse::<i32>().unwrap();
        }
    }

    cals.sort();

    println!("max: {}", cals[cals.len()-1]);
    println!("sum of max three: {}", cals[cals.len()-3..cals.len()].iter().sum::<i32>())
}
