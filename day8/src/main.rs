use std::fs;
use std::collections::{HashMap, HashSet};

fn part_1(contents_split: &Vec<&str>) -> i32 {
    let mut count: i32 = 0;
    for i in contents_split.iter() {
        let parts: Vec<&str> = i.split('|').collect();
        let digits: Vec<&str> = parts[1].strip_prefix(' ').unwrap().split(' ').collect();
        for j in digits.iter() {
            match j.len() {
               2 | 3 | 4 | 7 => { count += 1 }, 
                _ => {}
            }
        }
    }
    count
}

fn segment_mapping(wires: Vec<&str>, digits: Vec<&str>) -> i32 {
    let mut num: i32 = 0;
    let mut num_to_set: HashMap<i32, HashSet<char>> = HashMap::new();
    let mut mapping: HashMap<char, char> = HashMap::new();
    let mut counter: Vec<u8> = vec![0; 7];

    for i in wires.iter() {
        match i.len() {
            2 => {
                let temp: HashSet<char> = i.chars().collect();
                num_to_set.insert(1, temp);
            },
            3 => {
                let temp: HashSet<char> = i.chars().collect();
                num_to_set.insert(7, temp);
            },
            4 => {
                let temp: HashSet<char> = i.chars().collect();
                num_to_set.insert(4, temp);
            },
            _ => {},
        }
        for j in i.chars() {
            let index = j as usize - 97;
            counter[index] += 1;
        }
    }
    let a = num_to_set.get(&7).unwrap().difference(num_to_set.get(&1).unwrap()).next().unwrap();
    mapping.insert('a', (*a).clone());
    for (i, item) in counter.iter().enumerate() {
        match item {
            4 => {
                let c = (i as u8 + 97) as char;
                mapping.insert('e', c);
            },
            6 => {
                let c = (i as u8 + 97) as char;
                mapping.insert('b', c);
            },
            9 => {
                let c = (i as u8 + 97) as char;
                mapping.insert('f', c);
            },
            8 => {
                let c = (i as u8 + 97) as char;
                if !c.eq(mapping.get(&'a').unwrap()) { mapping.insert('c', c); }
            },
            7 => {
                let c = (i as u8 + 97) as char;
                if num_to_set.get(&4).unwrap().contains(&c) { mapping.insert('d', c); }
                else { mapping.insert('g', c); }
            },
            _ => {},
        }
    }
    let nums_str = vec!["abcefg", "cf", "acdeg", "acdfg", "bcdf",
        "abdfg", "abdefg", "acf", "abcdefg", "abcdfg"];
    let f = |x: char| (*mapping.get(&x).unwrap()).clone();
    for (i, item) in nums_str.iter().enumerate() {
        let temp: HashSet<char> = item.chars().map(f).collect();
        num_to_set.insert(i as i32, temp);
    }
    for i in digits.iter() {
        for j in 0..=9 {
            let temp: HashSet<char> = i.chars().collect();
            if num_to_set.get(&j).unwrap().eq(&temp) {
                num = num * 10 + j;
                break;
            }
        }
    }
    num
}

fn part_2(contents_split: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    for i in contents_split.iter() {
        let parts: Vec<&str> = i.split('|').collect();
        let wires: Vec<&str> = parts[0].strip_suffix(' ').unwrap().split(' ').collect();
        let digits: Vec<&str> = parts[1].strip_prefix(' ').unwrap().split(' ').collect();
        sum += segment_mapping(wires, digits);
    }
    sum
}



fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();

    println!("{}", part_1(&contents_split));
    println!("{}", part_2(&contents_split));
}
