use std::fs;
use std::collections::{HashSet};

#[derive(Hash)]
struct Point (i32, i32);
struct Fold (u8, i32);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}

fn fold(points: &Vec<Point>, fold: &Fold) -> Vec<Point> {
    let f = |x: &Point| {
        if fold.0 == 0 { Point(fold.1 - (x.0 - fold.1).signum() * (x.0 - fold.1), x.1) }
        else { Point(x.0, fold.1 - (x.1 - fold.1).signum() * (x.1 - fold.1)) }
    };
    let result: HashSet<Point> = points.iter().map(f).collect();
    result.into_iter().collect()
}

fn display_paper(points: &Vec<Point>, x_max: usize, y_max: usize) {
    let mut paper = vec![vec!["."; x_max]; y_max];
    for i in points.iter() {
        paper[i.1 as usize][i.0 as usize] = "#";
    }
    for i in paper.iter() {
        for j in i.iter() {
            print!("{}", j);
        }
        print!("\n");
    }
}


fn main() {
    let filename = String::from("input.txt");
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
    let mut result1 = fold(&points, &folds[0]);
    for i in folds.iter().skip(1) {
        let temp = fold(&result1, i);
        result1.clear();
        result1.extend(temp);
    }
    let x_max = result1.iter().fold(0, |acc, x| if acc > x.0 { acc } else { x.0 } );
    let y_max = result1.iter().fold(0, |acc, x| if acc > x.1 { acc } else { x.1 } );
    display_paper(&result1, (x_max + 1) as usize, (y_max + 1) as usize);
}
