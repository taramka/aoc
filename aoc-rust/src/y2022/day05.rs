use std::fs;

fn parse_command(input: &str) -> (usize, usize, usize) {
    let split = input.split(" ").collect::<Vec<&str>>();

    assert!(split[0] == "move");
    let amount = split[1].parse::<usize>()
        .expect("amount of crates is not numeric");

    assert!(split[2] == "from");
    let from = split[3].parse::<usize>()
        .expect("source crate stack id is not numeric");

    assert!(split[4] == "to");
    let to = split[5].parse::<usize>()
        .expect("target crate stack id is not numeric");

    (amount, from, to)
}

pub fn main() {
    let contents = fs::read_to_string("inputs/day05.txt")
        .expect("could not read the file");

    let crate1: Vec<char> = vec!['D', 'B', 'J', 'V'];
    let crate2: Vec<char> = vec!['P', 'V', 'B', 'W', 'R', 'D', 'F'];
    let crate3: Vec<char> = vec!['R', 'G', 'F', 'L', 'D', 'C', 'W', 'Q'];
    let crate4: Vec<char> = vec!['W', 'J', 'P', 'M', 'L', 'N', 'D', 'B'];
    let crate5: Vec<char> = vec!['H', 'N', 'B', 'P', 'C', 'S', 'Q'];
    let crate6: Vec<char> = vec!['R', 'D', 'B', 'S', 'N', 'G'];
    let crate7: Vec<char> = vec!['Z', 'B', 'P', 'M', 'Q', 'F', 'S', 'H'];
    let crate8: Vec<char> = vec!['W', 'L', 'F'];
    let crate9: Vec<char> = vec!['S', 'V', 'F', 'M', 'R'];
    
    let mut crates: Vec<Vec<char>> = vec![
        crate1, crate2, crate3, crate4, crate5, crate6, crate7, crate8, crate9
    ];


    for item in contents.lines() {
        let (amount, from, to) = parse_command(item);

        /*for _ in 0..amount {
            let c: char = crates[from - 1].pop().unwrap();
            crates[to - 1].push(c);
        }*/

        let mut holder = Vec::new();

        for _ in 0..amount {
            let c: char = crates[from - 1].pop().unwrap();
            holder.push(c);
        }
        for _ in 0..amount {
            let c: char = holder.pop().unwrap();
            crates[to - 1].push(c);
        }
    }

    for i in 0..9 {
        print!("{}", crates[i][crates[i].len() - 1]);
    }
    println!("");
}
