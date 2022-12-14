use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_str: String = fs::read_to_string(file_path).expect("couldn't read input");
    let mut four_char: [u8; 4] = [0; 4];
    let mut part_two: [u8; 26] = [0; 26];
    for i in 3..input_str.len() {
        four_char[0] = input_str.as_bytes()[i-3];
        four_char[1] = input_str.as_bytes()[i-2];
        four_char[2] = input_str.as_bytes()[i-1];
        four_char[3] = input_str.as_bytes()[i];
        if four_char[0] == four_char[1] || four_char[0] == four_char[2] || four_char[0] == four_char[3]
        || four_char[1] == four_char[2] || four_char[1] == four_char[3]
        || four_char[2] == four_char[3] {
            continue;
        }
        else {
            println!("{}", i+1);
            break;
        }
    }
    let mut invalid: bool = false;
    for i in 13..input_str.len() {
        for j in 0..26 {
            part_two[j] = 0;
        }
        invalid = false;
        for j in 0..14 {
            part_two[(input_str.as_bytes()[i-j] - b'a') as usize] += 1;
        }
        for j in 0..part_two.len() {
            if part_two[j] > 1 {
                invalid = true;
                break; 
            }
        }
        if invalid {
        }
        else {
            println!("Look here! {}", i+1);
            break;
        }
    }
}
