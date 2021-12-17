use std::fs;

struct Header {
    version: u8,
    id: u8,
}

enum Packet {
    OperatorPacket { header: Header, data: Vec<Packet> },
    LiteralPacket { header: Header, value: u64 },
}

fn to_binary(s: &str) -> String {
    let mut result = String::new();
    let binary_iter = s
        .chars()
        .map(|x: char| format!("{:04b}", x.to_digit(16).unwrap()));
    result.extend(binary_iter);
    result
}

fn decode_header(input: &str) -> Option<Header> {
    if input.len() < 11 {
        return None;
    }
    let v = u8::from_str_radix(&input[..3], 2).unwrap();
    let i = u8::from_str_radix(&input[3..6], 2).unwrap();
    let header = Header { version: v, id: i };
    return Some(header);
}

fn decode_literal(input: &str) -> (u64, usize) {
    let mut end = false;
    let mut counter: usize = 0;
    let mut result = String::new();
    let digits: Vec<char> = input.chars().collect();
    while !end {
        if digits[counter] == '0' {
            end = true;
        }
        for i in 0..4 {
            result.push(digits[counter + i + 1]);
        }
        counter += 5;
    }
    return (u64::from_str_radix(&result, 2).unwrap(), counter);
}

fn decode_operator(input: &str) -> (Vec<Packet>, usize) {
    let mut result: Vec<Packet> = Vec::new();
    let mut counter: usize = 0;
    let digits: Vec<char> = input.chars().collect();
    counter += 1;
    if digits[0] == '0' {
        let temp: String = digits[1..16].iter().collect();
        let length = u32::from_str_radix(&temp, 2).unwrap();
        counter += 15;
        while counter < 15 + length as usize {
            let (p, l) = decode_packet(&input[counter..16 + length as usize]);
            counter += l;
            result.push(p.unwrap());
        }
    } else {
        let temp: String = digits[1..12].iter().collect();
        let n_packets = u32::from_str_radix(&temp, 2).unwrap();
        counter += 11;
        for _ in 0..n_packets {
            let (p, l) = decode_packet(&input[counter..]);
            counter += l;
            result.push(p.unwrap());
        }
    }
    return (result, counter);
}

fn decode_packet(input: &str) -> (Option<Packet>, usize) {
    let result: Packet;
    let mut counter: usize = 0;
    let header = decode_header(input);
    match header {
        Some(x) => {
            counter += 6;
            if x.id == 4 {
                let (v, l) = decode_literal(&input[counter..]);
                result = Packet::LiteralPacket {
                    header: x,
                    value: v,
                };
                counter += l;
            } else {
                let (v, l) = decode_operator(&input[counter..]);
                result = Packet::OperatorPacket { header: x, data: v };
                counter += l;
            }
            return (Some(result), counter);
        }
        None => {
            return (None, 0);
        }
    }
}

fn part_1(p: &Packet) -> u32 {
    let mut sum: u32 = 0;
    match p {
        Packet::LiteralPacket {
            header: h,
            value: _,
        } => {
            sum += h.version as u32;
        }
        Packet::OperatorPacket { header: h, data: d } => {
            sum += h.version as u32;
            for i in d.iter() {
                sum += part_1(i);
            }
        }
    }
    sum
}

fn part_2(p: &Packet) -> u64 {
    let mut sum: u64 = 0;
    match p {
        Packet::LiteralPacket {
            header: _,
            value: v,
        } => {
            sum = *v;
        }
        Packet::OperatorPacket { header: h, data: d } => match h.id {
            0 => {
                for i in d.iter() {
                    sum += part_2(i);
                }
            }
            1 => {
                sum = 1;
                for i in d.iter() {
                    sum *= part_2(i);
                }
            }
            2 => {
                let mut min = u64::MAX;
                for i in d.iter() {
                    let temp = part_2(i);
                    if temp < min {
                        min = temp;
                    }
                }
                sum = min;
            }
            3 => {
                let mut max = 0;
                for i in d.iter() {
                    let temp = part_2(i);
                    if temp > max {
                        max = temp;
                    }
                }
                sum = max;
            }
            5 => {
                let p1 = part_2(&d[0]);
                let p2 = part_2(&d[1]);
                sum = if p1 > p2 { 1 } else { 0 };
            }
            6 => {
                let p1 = part_2(&d[0]);
                let p2 = part_2(&d[1]);
                sum = if p1 < p2 { 1 } else { 0 };
            }
            7 => {
                let p1 = part_2(&d[0]);
                let p2 = part_2(&d[1]);
                sum = if p1 == p2 { 1 } else { 0 };
            }
            _ => (),
        },
    }
    sum
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let binary_string = to_binary(&contents[..]);
    let h = decode_packet(&binary_string).0.unwrap();
    println!("{}", part_1(&h));
    println!("{}", part_2(&h));
}
