use std::fs;
use std::collections::{HashSet};

#[derive(Debug)]
struct Point (i32, i32);
#[derive(Debug)]
struct Fold (u8, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

fn main() {
    let filename = String::from("sample");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();
    let f = |x: &str| x.parse::<i32>().unwrap();

    let mut points: Vec<Point> = vec![];
    let mut folds: Vec<Fold> = vec![];
    let mut change = false;
    for i in content_lines.iter() {
        if i.eq(&"") { 
            change = true;
            continue;
        }
        if !change {
            let coors:Vec<i32> = i.split(',').map(f).collect();
            points.push(Point(coors[0], coors[1]));
        }
        else {
            let fold: Vec<&str> = i.split(' ').skip(2).next().unwrap().split('=').collect();
            if fold[0].eq("x") {
                folds.push(Fold(0, f(fold[1])));
            }
            else {
                folds.push(Fold(1, f(fold[1])));
            }
        }
    }
    println!("{:?}", points);
    println!("{:?}", folds);
    
}
