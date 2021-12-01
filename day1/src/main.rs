use std::fs;

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();
    // Part 1
    let mut count: usize = 0;
    let mut last: i32 = i32::max_value();
    for i in contents_split.iter() {
        let num = i.parse::<i32>().unwrap();
        if  num > last { count += 1; }
        last = num;
    }
    println!("{}", count);
    // Part 2
    let mut count: usize = 0;
    let mut last: i32 = i32::max_value();
    for i in 0..=(contents_split.len() - 3) {
        let num = contents_split[i].parse::<i32>().unwrap() + 
            contents_split[i + 1].parse::<i32>().unwrap() +
            contents_split[i + 2].parse::<i32>().unwrap();
        if  num > last { count += 1; }
        last = num;
    }
    println!("{}", count);
}
