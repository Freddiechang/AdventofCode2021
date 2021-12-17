use std::collections::{HashMap, HashSet};
use std::fs;

struct Position {
    b: usize,
    x: usize,
    y: usize,
}

fn part1(
    numbers: &Vec<i32>,
    locations: &HashMap<i32, Vec<Position>>,
    boards: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    let mut status: Vec<Vec<i32>> = vec![vec![0; 10]; boards.len()];
    let mut board_win: i32 = -1;
    let mut current_num = -1;
    for i in numbers.iter() {
        let l = locations.get(i);
        current_num = *i;
        match l {
            Some(x) => {
                for j in x.iter() {
                    status[j.b][j.x] += 1;
                    if status[j.b][j.x] == 5 {
                        board_win = j.b as i32;
                    }
                    status[j.b][j.y + 5] += 1;
                    if status[j.b][j.y + 5] == 5 {
                        board_win = j.b as i32;
                    }
                    boards[j.b][j.y][j.x] = -1;
                }
            }
            None => (),
        }
        if board_win != -1 {
            break;
        }
    }
    let mut sum: i32 = 0;
    if board_win != -1 {
        for i in 0..5 {
            for j in 0..5 {
                if boards[board_win as usize][i][j] != -1 {
                    sum += boards[board_win as usize][i][j];
                }
            }
        }
    }
    return sum * current_num;
}

fn part2(
    numbers: &Vec<i32>,
    locations: &HashMap<i32, Vec<Position>>,
    boards: &mut Vec<Vec<Vec<i32>>>,
) -> i32 {
    let mut status: Vec<Vec<i32>> = vec![vec![0; 10]; boards.len()];
    let mut board_win_set: HashSet<usize> = HashSet::new();
    let mut current_num = -1;
    let mut sum: i32 = 0;
    for i in numbers.iter() {
        let l = locations.get(i);
        match l {
            Some(x) => {
                for j in x.iter() {
                    boards[j.b][j.y][j.x] = -1;
                    status[j.b][j.x] += 1;
                    if status[j.b][j.x] == 5 && board_win_set.insert(j.b) {
                        sum = 0;
                        current_num = *i;
                        for m in 0..5 {
                            for n in 0..5 {
                                if boards[j.b][m][n] != -1 {
                                    sum += boards[j.b][m][n];
                                }
                            }
                        }
                    }
                    status[j.b][j.y + 5] += 1;
                    if status[j.b][j.y + 5] == 5 && board_win_set.insert(j.b) {
                        sum = 0;
                        current_num = *i;
                        for m in 0..5 {
                            for n in 0..5 {
                                if boards[j.b][m][n] != -1 {
                                    sum += boards[j.b][m][n];
                                }
                            }
                        }
                    }
                    boards[j.b][j.y][j.x] = -1;
                }
            }
            None => (),
        }
    }
    return sum * current_num;
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let contents_split: Vec<&str> = contents.split('\n').collect();

    let f = |x: &str| x.parse::<i32>().unwrap();
    let numbers: Vec<i32> = contents_split[0].split(',').map(f).collect();
    let mut locations: HashMap<i32, Vec<Position>> = HashMap::new();
    let mut boards: Vec<Vec<Vec<i32>>> = vec![vec![vec![0; 5]; 5]; (contents_split.len() - 2) / 6];
    let mut y: usize = 0;
    for (i, item) in contents_split[2..].iter().enumerate() {
        if i % 6 == 5 {
            continue;
        }
        let nums_line: Vec<i32> = item.split_ascii_whitespace().map(f).collect();
        for (k, num) in nums_line.iter().enumerate() {
            let temp_pos = Position {
                b: i / 6,
                x: k,
                y: y,
            };
            boards[i / 6][y][k] = *num;
            let location = locations.get_mut(num);
            match location {
                None => {
                    locations.insert(*num, vec![temp_pos]);
                }
                Some(t) => t.push(temp_pos),
            }
        }
        y = (y + 1) % 5;
    }

    let result1 = part1(&numbers, &locations, &mut (boards.clone()));
    let result2 = part2(&numbers, &locations, &mut (boards.clone()));
    println!("{}", result1);
    println!("{}", result2);
}
