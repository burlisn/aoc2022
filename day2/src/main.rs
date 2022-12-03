use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_str = fs::read_to_string(file_path).expect("Should have been able to read this file");
    let op_rock = b'A';
    let me_rock = b'X';
    let mut score: u32 = 0;
    let mut score2: u32 = 0;
    for line in input_str.lines() {
        let op = line.chars().nth(0).unwrap() as u8 - op_rock;
        let me = line.chars().nth(2).unwrap() as u8 - me_rock;

        score += (me + 1) as u32;

        if ( me + 1 ) % 3 == op { //I lose
            score += 0;
        }
        else if op == me { //tie
            score += 3;
        }
        else { //I win
            score += 6;
        }

        if me == 0 {
            score2 += (op + 2) as u32 % 3 + 1;
        }
        else if me == 1 {
            score2 += op as u32 + 1;
            score2 += 3;
        }
        else {
            score2 += (op + 1) as u32 % 3 + 1;
            score2 += 6;
        }
        println!("{} {}", op, me);
    }

    
    let char_val = b'A'; //Rock
    println!("{}", char_val);
    let char_val = b'B'; //Paper
    println!("{}", char_val);
    let char_val = b'C'; //Scissors
    println!("{}", char_val);
    let char_val = b'X'; //Rock
    println!("{}", char_val);
    let char_val = b'Y'; //Paper
    println!("{}", char_val);
    let char_val = b'Z'; //Scissors
    println!("{}", char_val);
    if char_val as u8 == 90 {
        println!("BOOYAH");
    }
    println!("{}", score);
    println!("{}", score2);
}
