use std::env;
use std::fs;

fn main() {
    println!("Hello, world!");
    let file_path = "input.txt";
    let mut run_sum = 0;
    let mut highest_cal = 0;
    let mut second = 0;
    let mut third = 0;
    println!("In file {}", file_path);
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    for line in contents.lines() {
        //println!("{}", line);
        if(line.is_empty()) {
            //println!("TRUE");
            if run_sum > highest_cal {
                third = second;
                second = highest_cal;
                highest_cal = run_sum;
            }
            else if run_sum > second {
                third = second;
                second = run_sum; 
            }
            else if run_sum > third {
                third = run_sum;
            }
            run_sum = 0;
        }        
        else {
            //println!("{}", line);
            run_sum = run_sum + line.parse::<i32>().unwrap();
        }
    }
    println!("{}", highest_cal);
    println!("{}", highest_cal + second + third);
}
