use std::fs;

fn decode(token: &str) -> Option<i32> {
    match token {
        "A" => Some(1),
        "B" => Some(2),
        "C" => Some(3),
        "X" => Some(1),
        "Y" => Some(2),
        "Z" => Some(3),
        _ => None,
    }
}

fn get_score(token: &str) -> Option<i32> {
    match token {
        "X" => Some(0),
        "Y" => Some(3),
        "Z" => Some(6),
        _ => None,
    }
}

fn get_loose(token: &str) -> Option<&str> {
    match token {
        "A" => Some("Z"),
        "B" => Some("X"),
        "C" => Some("Y"),
        _ => None,
    }
}

fn get_win(token: &str) -> Option<&str> {
    match token {
        "A" => Some("Y"),
        "B" => Some("Z"),
        "C" => Some("X"),
        _ => None,
    }
}

fn calculate_score(fst: &str, snd: &str) -> i32 {
    if decode(fst) == decode(snd) {
        return 3 + decode(snd).expect("unexpected character occurred");
    }

    if fst == "A" {
        return if snd == "Y" {decode(snd).unwrap() + 6} else {decode(snd).expect("unexpected character occurred")};
    }
    
    if fst == "B" {
        return if snd == "Z" {decode(snd).unwrap() + 6} else {decode(snd).expect("unexpected character occurred")};
    }
    
    if snd == "X" {decode(snd).unwrap() + 6} else {decode(snd).expect("unexpected character occurred")}
}

fn calculate_strategy(fst: &str, snd: &str) -> Option<i32> {
    match snd {
        "X" => {
            let chosen = get_loose(fst).expect("unexpected character occurred");
            Some(get_score(snd).unwrap() + decode(chosen).unwrap())
        }
        "Y" => Some(get_score(snd).unwrap() + decode(fst).expect("unexpected character occurred")),
        "Z" => {
            let chosen = get_win(fst).expect("unexpected character occurred");
            Some(get_score(snd).unwrap() + decode(chosen).unwrap())
        }
        _ => None,
    }
}

pub fn main() {
    let contents = fs::read_to_string("inputs/day02.txt")
        .expect("could not read the file");

    let mut score: i32 = 0;
    let mut strategy_score: i32 = 0;

    for item in contents.lines() {
        let split = item.split(" ").collect::<Vec<&str>>();
        score += calculate_score(split[0], split[1]);
        strategy_score += calculate_strategy(split[0], split[1]).expect("unexpected character occurred");
    }

    println!("fst strategy: {}", score);
    println!("snd strategy: {}", strategy_score);
}
