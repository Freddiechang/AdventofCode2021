use std::fs;

#[derive(Clone)]
enum SNumber {
    Pair(Box<SNumber>, Box<SNumber>),
    Number(u32),
}

impl SNumber {
    fn parse(s: &[u8]) -> (SNumber, &[u8]) {
        match s[0] {
            b'[' => {
                let (left, i) = Self::parse(&s[1..]);
                let (right, i) = Self::parse(i);
                return (SNumber::Pair(Box::new(left), Box::new(right)), i);
            }
            x @ b'0'..=b'9' => {
                return (SNumber::Number((x - b'0') as u32), &s[1..]);
            }
            _ => {
                return Self::parse(&s[1..]);
            }
        }
    }
}

fn explode(p: &mut SNumber, n: usize) -> (bool, Option<u32>, Option<u32>) {
    match p {
        SNumber::Number(_) => {
            return (false, None, None);
        }
        SNumber::Pair(l, r) => {
            if n > 3 {
                match (&**l, &**r) {
                    (SNumber::Number(x1), SNumber::Number(x2)) => {
                        let res = (true, Some(*x1), Some(*x2));
                        *p = SNumber::Number(0);
                        return res;
                    }
                    _ => {}
                }
            }
            let (s, x1, x2) = explode(&mut *l, n + 1);
            if s {
                propagate_right(&mut *r, x2);
                return (true, x1, None);
            }
            let (s, x1, x2) = explode(&mut *r, n + 1);
            if s {
                propagate_left(&mut *l, x1);
                return (true, None, x2);
            }
        }
    }
    (false, None, None)
}

fn propagate_left(p: &mut SNumber, n: Option<u32>) {
    if let Some(n) = n {
        match p {
            SNumber::Number(i) => {
                *p = SNumber::Number(n + *i);
            }
            SNumber::Pair(_, r) => {
                propagate_left(&mut *r, Some(n));
            }
        }
    }
}

fn propagate_right(p: &mut SNumber, n: Option<u32>) {
    if let Some(n) = n {
        match p {
            SNumber::Number(i) => {
                *p = SNumber::Number(n + *i);
            }
            SNumber::Pair(l, _) => {
                propagate_right(&mut *l, Some(n));
            }
        }
    }
}

fn split(p: &mut SNumber) -> bool {
    match p {
        SNumber::Number(i) => {
            if *i > 9 {
                let left = *i / 2;
                let right = *i - left;
                *p = SNumber::Pair(
                    Box::new(SNumber::Number(left)),
                    Box::new(SNumber::Number(right)),
                );
                return true;
            }
            return false;
        }
        SNumber::Pair(l, r) => {
            return split(&mut *l) || split(&mut *r);
        }
    }
}

fn reduce(p: &mut SNumber) {
    let f = |x: &mut SNumber| explode(x, 0).0;
    loop {
        while f(p) {}
        if !split(p) {
            break;
        }
    }
}

fn add(p1: SNumber, p2: SNumber) -> SNumber {
    let mut x = SNumber::Pair(Box::new(p1), Box::new(p2));
    reduce(&mut x);
    return x;
}

fn magnitude(p: &SNumber) -> u32 {
    match p {
        SNumber::Number(x) => *x,
        SNumber::Pair(l, r) => 3 * magnitude(l) + 2 * magnitude(r),
    }
}

fn part_1(lines: &Vec<&str>) -> u32 {
    let result = lines
        .iter()
        .map(|x| SNumber::parse(x.as_bytes()).0)
        .reduce(|acc, item| add(acc, item))
        .unwrap();
    magnitude(&result)
}

fn part_2(lines: &Vec<&str>) -> u32 {
    let numbers: Vec<SNumber> = lines
        .iter()
        .map(|x| SNumber::parse(x.as_bytes()).0)
        .collect();
    let mut max: u32 = 0;
    for i in 0..numbers.len() {
        for j in 0..numbers.len() {
            if i != j {
                let r = magnitude(&add(numbers[i].clone(), numbers[j].clone()));
                if r > max {
                    max = r;
                }
            }
        }
    }
    max
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();
    println!("{}", part_1(&content_lines));
    println!("{}", part_2(&content_lines));
}
