use std::collections::HashMap;
use std::fs;

fn create_hashmap(timers: &Vec<i32>) -> HashMap<i32, u64> {
    let mut timers_map: HashMap<i32, u64> = HashMap::new();
    for i in 0..=8 {
        timers_map.insert(i, 0);
    }
    for i in timers.iter() {
        *timers_map.get_mut(i).unwrap() += 1;
    }
    timers_map
}

fn next_day(timers_map: &mut HashMap<i32, u64>) {
    let temp: u64 = *timers_map.get_mut(&0).unwrap();
    for i in 1..=8 {
        *timers_map.get_mut(&(i - 1)).unwrap() = *timers_map.get_mut(&i).unwrap();
    }
    *timers_map.get_mut(&6).unwrap() += temp;
    *timers_map.get_mut(&8).unwrap() = temp;
}

fn total(timers_map: &HashMap<i32, u64>) -> u64 {
    let mut count: u64 = 0;
    for i in 0..=8 {
        count += *timers_map.get(&i).unwrap();
    }
    count
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let f = |x: &str| x.parse::<i32>().unwrap();
    let numbers: Vec<i32> = contents.split(',').map(f).collect();

    let mut timers_map = create_hashmap(&numbers);
    for _ in 0..256 {
        next_day(&mut timers_map);
    }
    println!("{}", total(&timers_map));
}
