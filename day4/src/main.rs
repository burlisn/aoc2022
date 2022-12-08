use std::fs;

fn main() {
    let file_path = "input.txt";
    let input_str = fs::read_to_string(file_path).expect("couldn't read input");    
    let mut ans1 = 0;
    let mut ans2 = 0;
    for line in input_str.lines() {
        let v: Vec<&str> = line.split(|c: char| c.is_ascii_punctuation()).collect();
//        println!("{:?}", v);
        let al: i32 = v[0].parse().unwrap();
        let au: i32 = v[1].parse().unwrap();
        let bl: i32 = v[2].parse().unwrap();
        let bu: i32 = v[3].parse().unwrap();
        if (al <= bl && au >= bu) || (bl <= al && bu >= au) {
            ans1 += 1;
        }
        if au >= bl && bu >= al {
            ans2 += 1;
        }
    }
    println!("{}", ans1);
    println!("{}", ans2);
}
