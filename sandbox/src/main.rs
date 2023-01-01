fn main() {
    let mut my_str: String = String::from("hello, this is my string");    
    let immut_ref: &String = &my_str;
    let mut_ref: &mut String = &mut my_str;
    mut_ref.push_str(". Here we are now");
}

fn longest(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    }
    else {
        y
    }
}
