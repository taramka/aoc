use std::fs;

fn get_borders(range: &str) -> (i32, i32) {
    let borders = range.split("-").collect::<Vec<&str>>();

    (borders[0].parse::<i32>().unwrap(), borders[1].parse::<i32>().unwrap())
}

fn full_overlap(fst: (i32, i32), snd: (i32, i32)) -> bool {
    (fst.0 <= snd.0 && fst.1 >= snd.1) || (snd.0 <= fst.0 && snd.1 >= fst.1)
}

fn overlap(fst: (i32, i32), snd: (i32, i32)) -> bool {
    full_overlap(fst, snd)
    || (fst.0 >= snd.0 && fst.0 <= snd.1)
    || (fst.1 >= snd.0 && fst.1 <= snd.1)
}

pub fn main() {
    let contents = fs::read_to_string("inputs/day04.txt")
        .expect("could not read the file");

    let mut full_overlaps: i32 = 0;
    let mut overlaps: i32 = 0;

    for item in contents.lines() {
        let ranges = item.split(",").collect::<Vec<&str>>();
        let fst = get_borders(ranges[0]);
        let snd = get_borders(ranges[1]);

        if full_overlap(fst, snd) {
            full_overlaps += 1;
        }

        if overlap(fst, snd) {
            overlaps += 1;
        }
    }

    println!("full overlaps: {}", full_overlaps);
    println!("total overlaps: {}", overlaps);
}
