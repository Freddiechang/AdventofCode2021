use std::fs;

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();
    let digits = contents_split[0].len();
    let total = contents_split.len();
    let mut count1: Vec<u32> = vec![0; digits];
    for i in contents_split.iter() {
        let bytes = i.as_bytes();
        for j in 0..i.len() {
            if bytes[j].eq(&('1' as u8)) { count1[j] += 1; }
        }
    }
    // part 1
    let mut gamma: i32 = 0;
    let mut epsilon: i32 = 0;
    for i in 0..digits {
        let count = count1[i];
        if count > (total as u32 - count) {
            gamma = (gamma << 1) + 1;
            epsilon = epsilon << 1;
        }
        else {
            gamma = gamma << 1;
            epsilon = (epsilon << 1) + 1;
        }
    }
    println!("{}", gamma * epsilon);
    // part 2
    let mut indices_o2: Vec<usize> = (0..contents_split.len()).collect();
    let mut indices_co2: Vec<usize> = (0..contents_split.len()).collect();
    let mut keep: Vec<bool> = vec![];
    let mut count1: usize;
    for i in 0..digits {
        count1 = 0;
        if indices_o2.len() != 1 {
            for j_o2 in indices_o2.iter() {
                let bytes = contents_split[*j_o2].as_bytes();
                if bytes[i].eq(&('1' as u8)) { count1 += 1; }
            }
            if count1 >= indices_o2.len() - count1 {
                for k in 0..indices_o2.len() {
                    let bytes = contents_split[indices_o2[k]].as_bytes();
                    if bytes[i].eq(&('1' as u8)) { keep.push(true); }
                    else { keep.push(false); }
                }
                let mut keep_iter = keep.iter();
                indices_o2.retain(|_| *keep_iter.next().unwrap());
                keep.clear();
            }
            else
            {
                for k in 0..indices_o2.len() {
                    let bytes = contents_split[indices_o2[k]].as_bytes();
                    if bytes[i].eq(&('0' as u8)) { keep.push(true); }
                    else { keep.push(false); }
                }
                let mut keep_iter = keep.iter();
                indices_o2.retain(|_| *keep_iter.next().unwrap());
                keep.clear();
            }
        }
        count1 = 0;
        if indices_co2.len() != 1 {
            for j_co2 in indices_co2.iter() {
                let bytes = contents_split[*j_co2].as_bytes();
                if bytes[i].eq(&('1' as u8)) { count1 += 1; }
            }
            if count1 < indices_co2.len() - count1 {
                for k in 0..indices_co2.len() {
                    let bytes = contents_split[indices_co2[k]].as_bytes();
                    if bytes[i].eq(&('1' as u8)) { keep.push(true); }
                    else { keep.push(false); }
                }
                let mut keep_iter = keep.iter();
                indices_co2.retain(|_| *keep_iter.next().unwrap());
                keep.clear();
            }
            else
            {
                for k in 0..indices_co2.len() {
                    let bytes = contents_split[indices_co2[k]].as_bytes();
                    if bytes[i].eq(&('0' as u8)) { keep.push(true); }
                    else { keep.push(false); }
                }
                let mut keep_iter = keep.iter();
                indices_co2.retain(|_| *keep_iter.next().unwrap());
                keep.clear();
            }
        }
    }
    let o2 = i32::from_str_radix(contents_split[indices_o2[0]], 2).unwrap();
    let co2 = i32::from_str_radix(contents_split[indices_co2[0]], 2).unwrap();
    println!("{}", o2 * co2);
}
