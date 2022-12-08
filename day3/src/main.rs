use std::fs;

fn main() {
    let mut three_track = 0;
    let mut one_string = "";
    let mut two_string = "";
    let mut three_string = "";
    let file_path = "input.txt";
    let input_str = fs::read_to_string(file_path).expect("Should have been able to read this file");
    let mut prio_sum: i32 = 0; 
    let mut prio_sum2: i32 = 0;
    for line in input_str.lines() {
        let mut pasta = false;
        let mid = line.len() / 2;
        let (first, second) = line.split_at(mid);
        for item1 in first.chars() {
            if pasta {
                break;
            }
            for item2 in second.chars() {
                if item1 == item2 {
                    prio_sum += get_char_value(item1); 
                    pasta = true;
                    break;
                }
            }
        }

        /* Part 2 */
        let mut sauce = false;
        match three_track {
            0 => one_string = line,
            1 => two_string = line,
            2 => three_string = line,
            _ => {}, 
        }
        three_track += 1;
        if three_track == 3 {
            for one_char in one_string.chars() {
                if sauce { break; }
                for two_char in two_string.chars() {
                    if sauce { break; }
                    if one_char != two_char {
                        continue;
                    }
                    for three_char in three_string.chars() {
                        if sauce { break; }
                        if one_char == two_char && two_char == three_char {
                            prio_sum2 += get_char_value(one_char); 
                            sauce = true;
                        }
                    }
                }
            }
            three_track = 0;
        }
    }
    println!("{}", prio_sum);
    println!("{}", prio_sum2);
}

fn get_char_value(item: char) -> i32 {
    if item >= 'a' {
        return item as i32 - 96;
    }
    else {
        return item as i32 - 38;
    }
}
