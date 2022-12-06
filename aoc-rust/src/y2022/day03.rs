use std::fs;

fn get_char_score(ch: char) -> u32 {
    if ch.is_lowercase() {
        return (ch as u32) - 96;
    }

    (ch as u32) - 38
}

fn get_line_score(line: &str) -> Option<u32> {
    let fst = &line[..line.len() / 2];
    let snd = &line[line.len() / 2..];

    for ch in fst.chars() {
        if snd.contains(ch) {
            return Some(get_char_score(ch));
        }
    }

    None
}

fn get_group_score(lines: &Vec<&str>) -> Option<u32> {
    for ch in lines[0].chars() {
        if lines[1].contains(ch) && lines[2].contains(ch) {
            return Some(get_char_score(ch));
        }
    }

    None
}

pub fn main() {
    let contents = fs::read_to_string("inputs/day03.txt")
        .expect("could not read the file");

    let mut total_score: u32 = 0;
    let mut group_score: u32 = 0;
    let mut group_counter: u8 = 0;
    let mut group = Vec::new();

    for rucksack in contents.lines() {
        group_counter += 1;
        group.push(rucksack);
        total_score += get_line_score(rucksack).expect("no item type in both compartments");

        if group_counter == 3 {
            group_score += get_group_score(&group).expect("no badge found in the group");
            group.clear();
            group_counter = 0;
        }
    }

    println!("total: {}", total_score);
    println!("group: {}", group_score);
}
