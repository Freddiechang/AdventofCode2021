use std::fs;

struct Point {
    x: usize,
    y: usize,
}

fn mark_line_part1(p1: Point, p2: Point, map: &mut Vec<Vec<i32>>) {
    if p1.x == p2.x {
        let lb = if p1.y > p2.y { p2.y } else { p1.y };
        let ub = if p1.y > p2.y { p1.y } else { p2.y };
        for i in lb..=ub {
            map[i][p1.x] += 1;
        }
    }
    else if p1.y == p2.y {
        let lb = if p1.x > p2.x { p2.x } else { p1.x };
        let ub = if p1.x > p2.x { p1.x } else { p2.x };
        for i in lb..=ub {
            map[p1.y][i] += 1;
        }
    }
}

fn mark_line_part2(p1: Point, p2: Point, map: &mut Vec<Vec<i32>>) {
    if p1.x != p2.x && p1.y != p2.y {
        let mut x = p1.x as i32;
        let mut y = p1.y as i32;
        let d_x: i32 = if p1.x > p2.x { -1 } else { 1 };
        let d_y: i32 = if p1.y > p2.y { -1 } else { 1 };
        while x != p2.x as i32 + d_x{
            map[y as usize][x as usize] += 1;
            x += d_x;
            y += d_y;
        }
    }
}


fn count_greater_2(map: &Vec<Vec<i32>>) -> i32 {
    let mut count: i32 = 0;
    for i in map.iter() {
        for j in i.iter() {
            if *j >= 2 { count += 1; }
        }
    }
    count
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();
    let f = |x: &str| x.parse::<usize>().unwrap();

    // part 1 and part 2
    let mut vent_map = vec![vec![0; 1000]; 1000];
    for line in contents_split.iter() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let p1: Vec<usize> = points[0].split(',').map(f).collect();
        let p2: Vec<usize> = points[1].split(',').map(f).collect();
        mark_line_part1(Point{x: p1[0], y: p1[1]}, Point{x: p2[0], y: p2[1]}, &mut vent_map);
    }
    println!("{}", count_greater_2(&vent_map));
    for line in contents_split.iter() {
        let points: Vec<&str> = line.split(" -> ").collect();
        let p1: Vec<usize> = points[0].split(',').map(f).collect();
        let p2: Vec<usize> = points[1].split(',').map(f).collect();
        mark_line_part2(Point{x: p1[0], y: p1[1]}, Point{x: p2[0], y: p2[1]}, &mut vent_map);
    }
    println!("{}", count_greater_2(&vent_map));

}
