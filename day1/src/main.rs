use std::fs;

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let f = |x: &str| x.parse::<i32>().unwrap();
    let nums: Vec<i32> = contents.split('\n').map(f).collect();
    // Part 1
    let mut count: usize = 0;
    let mut last: i32 = i32::max_value();
    for i in nums.iter() {
        if  *i > last { count += 1; }
        last = *i;
    }
    println!("{}", count);
    // Part 2
    let mut count: usize = 0;
    let mut last: i32 = i32::max_value();
    for i in 0..=(nums.len() - 3) {
        let num = nums[i] + 
            nums[i + 1] +
            nums[i + 2];
        if  num > last { count += 1; }
        last = num;
    }
    println!("{}", count);
}
