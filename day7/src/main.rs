use std::fs;

fn total_fuel_part1(numbers: &Vec<i32>, pos: i32) -> i32 {
    let mut count: i32 = 0;
    let mut total: i32 = 0;
    for i in numbers.iter() {
        if *i < pos {
            count += 1;
            total -= *i;
        } else {
            count -= 1;
            total += *i;
        }
    }
    total += count * pos;
    total
}

fn total_fuel_part2(numbers: &Vec<i32>) -> i32 {
    let sum: i32 = numbers.iter().sum();
    let pos: i32 = (sum as f32 / numbers.len() as f32).round() as i32;
    let mut total: i32 = i32::MAX;
    for i in pos - 1..=pos + 1 {
        let mut temp: i32 = 0;
        for j in numbers.iter() {
            let d = if *j - i >= 0 { *j - i } else { i - *j };
            temp += d * (d + 1) / 2;
        }
        if temp < total {
            total = temp;
        }
    }
    total
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let f = |x: &str| x.parse::<i32>().unwrap();
    let mut numbers: Vec<i32> = contents.split(',').map(f).collect();

    numbers.sort();
    let mut total: i32 = i32::MAX;
    if numbers.len() % 2 == 0 {
        let x1: usize = numbers.len() / 2 - 1;
        let x2: usize = x1 + 1;

        if x1 != x2 {
            for i in numbers[x1]..=numbers[x2] {
                let temp = total_fuel_part1(&numbers, i);
                if temp < total {
                    total = temp;
                }
            }
        }
    } else {
        let x1: usize = numbers.len() / 2 - 1;
        total = total_fuel_part1(&numbers, numbers[x1]);
    }
    println!("{}", total);
    println!("{}", total_fuel_part2(&numbers));
}
