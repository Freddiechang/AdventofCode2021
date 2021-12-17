use std::fs;

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();
    let f = |x: &str| x.parse::<i32>().unwrap();
    let mut h: i32 = 0;
    let mut d: i32 = 0;
    // part 1
    for i in contents_split.iter() {
        let tokens: Vec<&str> = i.split(' ').collect();
        let n = f(tokens[1]);
        if tokens[0].eq("forward") {
            h += n;
        } else if tokens[0].eq("down") {
            d += n;
        } else {
            d -= n;
        }
    }
    println!("{}", h * d);
    // part 2
    let mut h: i32 = 0;
    let mut d: i32 = 0;
    let mut aim: i32 = 0;
    for i in contents_split.iter() {
        let tokens: Vec<&str> = i.split(' ').collect();
        let n = f(tokens[1]);
        if tokens[0].eq("forward") {
            h += n;
            d += aim * n;
        } else if tokens[0].eq("down") {
            aim += n;
        } else {
            aim -= n;
        }
    }
    println!("{}", h * d);
}
