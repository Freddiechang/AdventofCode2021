use std::fs;

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();
    let mut h: i32 = 0;
    let mut d: i32 = 0;
    // part 1
    for i in contents_split.iter() {
        let tokens: Vec<&str> = i.split(' ').collect();
        if tokens[0].eq("forward") {
            h += tokens[1].parse::<i32>().unwrap();
        }
        else if tokens[0].eq("down") {
            d += tokens[1].parse::<i32>().unwrap();
        }
        else {
            d -= tokens[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", h * d);
    // part 2
    let mut h: i32 = 0;
    let mut d: i32 = 0;
    let mut aim: i32 = 0;
    for i in contents_split.iter() {
        let tokens: Vec<&str> = i.split(' ').collect();
        if tokens[0].eq("forward") {
            let f = tokens[1].parse::<i32>().unwrap();
            h += f;
            d += aim * f;

        }
        else if tokens[0].eq("down") {
            aim += tokens[1].parse::<i32>().unwrap();
        }
        else {
            aim -= tokens[1].parse::<i32>().unwrap();
        }
    }
    println!("{}", h * d);
}
