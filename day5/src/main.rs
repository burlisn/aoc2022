use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_str = fs::read_to_string(file_path).expect("couldn't read input");
    let mut ship: [Vec<char>; 9] =
    [
    vec!['S', 'M', 'R', 'N', 'W', 'J', 'V', 'T'],
    vec!['B', 'W', 'D', 'J', 'Q', 'P', 'C', 'V'],
    vec!['B', 'J', 'F', 'H', 'D', 'R', 'P'],
    vec!['F', 'R', 'P', 'B', 'M', 'N', 'D'],
    vec!['H', 'V', 'R', 'P', 'T', 'B'],
    vec!['C', 'B', 'P', 'T'],
    vec!['B', 'J', 'R', 'P', 'L'],
    vec!['N', 'C', 'S', 'L', 'T', 'Z', 'B', 'W'],
    vec!['L', 'S', 'G']
    ];
    let mut ship2: [Vec<char>; 9] =
    [
    vec!['S', 'M', 'R', 'N', 'W', 'J', 'V', 'T'],
    vec!['B', 'W', 'D', 'J', 'Q', 'P', 'C', 'V'],
    vec!['B', 'J', 'F', 'H', 'D', 'R', 'P'],
    vec!['F', 'R', 'P', 'B', 'M', 'N', 'D'],
    vec!['H', 'V', 'R', 'P', 'T', 'B'],
    vec!['C', 'B', 'P', 'T'],
    vec!['B', 'J', 'R', 'P', 'L'],
    vec!['N', 'C', 'S', 'L', 'T', 'Z', 'B', 'W'],
    vec!['L', 'S', 'G']
    ];

    for line in input_str.lines() {
        let v: Vec<&str> = line.split(' ').collect();
        let mut temp_stack: Vec<char> = vec![];
        let amount: i32 = v[1].parse().unwrap();
        let from_col: usize = v[3].parse().unwrap();
        let to_col: usize  = v[5].parse().unwrap();
        for i in 0..amount {
            let move_crate: char = ship[from_col-1].pop().unwrap();
            let move_crate2: char = ship2[from_col-1].pop().unwrap();
            ship[to_col-1].push(move_crate);
            temp_stack.push(move_crate2);
        }
        for i in 0..amount {
            let move_crate: char = temp_stack.pop().unwrap();
            ship2[to_col-1].push(move_crate);
        }
    }
    for i in 0..9 {
//        println!("{:?}", ship[i]);
        println!("{:?}", ship2[i]);
    }
}
