use std::fs;

struct File {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    size: u32,
    files: Vec<File>,
    sub_dirs: Vec<Dir>,
    parent_dir: Option<Box<Dir>>,
}

fn main() {
    let file_path = "input.txt";
    let input_str: String = fs::read_to_string(file_path).expect("couldn't read input");
    let mut top_dir = Dir {
        name: String::from("/"),
        size: 0,
        files: Vec::new(),
        sub_dirs: Vec::new(),
        parent_dir: None,
    };

    let mut cur_dir = &mut top_dir;

    for line in input_str.lines() {
        /* obtain line in parts */
        let line_parts: Vec<&str> = line.split(' ').collect();        

        /* determine line type */
        if line_parts[0] == "$" { /* command */
            println!("{}", line_parts[0]);

        }
        else if line_parts[0] == "dir" { /* dir output */
            println!("{}", line_parts[0]);
        }
        else { /* file output */
            println!("{}", line_parts[0]);
        }
    }
}
