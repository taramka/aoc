use std::collections::HashSet;
use std::fs;

const PACKET_MARKER_SIZE: usize = 4;
const MESSAGE_MARKER_SIZE: usize = 14;

fn valid_marker(content: &str, counter: usize, marker_size: usize) -> bool {
    if counter < marker_size {
        return false;
    }

    let s: HashSet<char> = content[counter - marker_size..counter].chars().collect();
    s.len() == marker_size
}

pub fn main() {
    let content = fs::read_to_string("inputs/day06.txt")
        .expect("could not read the file");

    let mut counter: usize = 0;
    let mut packet_found: bool = false;
    let mut message_found: bool = false;

    for _ in content.chars() {
        if packet_found == false && valid_marker(&content, counter, PACKET_MARKER_SIZE) {
            println!("packet marker at {}", counter);
            packet_found = true;
        }

        if message_found == false && valid_marker(&content, counter, MESSAGE_MARKER_SIZE) {
            println!("message marker at {}", counter);
            message_found = true;
        }

        if packet_found && message_found {
            break;
        }

        counter += 1;
    }
}
