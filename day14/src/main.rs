use std::collections::HashMap;
use std::fs;

fn insertion_step(
    count: HashMap<String, u64>,
    mappings: &HashMap<&str, &str>,
) -> HashMap<String, u64> {
    let mut new_count: HashMap<String, u64> = HashMap::new();
    let mut f = |x: String, v: u64| {
        if new_count.contains_key(&x[..]) {
            *new_count.get_mut(&x[..]).unwrap() += v;
        } else {
            new_count.insert(x, v);
        }
    };
    for (pair, v) in count.iter() {
        let elements: Vec<char> = pair.chars().collect();
        let e = mappings.get(&pair[..]).unwrap().chars().next().unwrap();
        let mut new_pair1 = String::from("");
        new_pair1.push(elements[0]);
        new_pair1.push(e);
        f(new_pair1, *v);
        let mut new_pair2 = String::from("");
        new_pair2.push(e);
        new_pair2.push(elements[1]);
        f(new_pair2, *v);
    }
    new_count
}

fn build_count(seq: &Vec<char>) -> HashMap<String, u64> {
    let mut count: HashMap<String, u64> = HashMap::new();
    for i in 0..seq.len() - 1 {
        let pair: String = seq[i..=i + 1].iter().collect();
        if count.contains_key(&pair) {
            *count.get_mut(&pair).unwrap() += 1;
        } else {
            count.insert(pair, 1);
        }
    }
    count
}

fn solution(original_seq: &str, mappings: &HashMap<&str, &str>, steps: u64) -> u64 {
    let seq: Vec<char> = original_seq.chars().collect();
    let first = seq[0];
    let last = seq[seq.len() - 1];
    let mut pairs = build_count(&seq);
    for _ in 0..steps {
        pairs = insertion_step(pairs, mappings);
    }
    let mut count: HashMap<char, u64> = HashMap::new();
    for (k, v) in pairs.iter() {
        let elements: Vec<char> = k.chars().collect();
        for i in elements.iter() {
            if count.contains_key(i) {
                *count.get_mut(i).unwrap() += *v as u64;
            } else {
                count.insert(*i, *v as u64);
            }
        }
    }
    for (k, v) in count.iter_mut() {
        if *k == first || *k == last {
            *v = (*v - 1) / 2 + 1;
        } else {
            *v /= 2;
        }
    }
    let mut max: u64 = 0;
    let mut min: u64 = u64::MAX;
    for (_, v) in count.iter() {
        if *v > max {
            max = *v;
        }
        if *v < min {
            min = *v;
        }
    }
    max - min
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();

    let original_seq = content_lines[0];
    let mut mappings: HashMap<&str, &str> = HashMap::new();
    for i in content_lines.iter().skip(2) {
        let mut iter = i.split(" -> ");
        mappings.insert(iter.next().unwrap(), iter.next().unwrap());
    }
    println!("{}", solution(&original_seq, &mappings, 40));
}
