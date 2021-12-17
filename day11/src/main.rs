use std::fs;

fn increment_neighbor(levels: &mut Vec<Vec<i32>>, flash_map: &mut Vec<Vec<bool>>, x: i32, y: i32) {
    if flash_map[x as usize][y as usize] {
        return ();
    }
    flash_map[x as usize][y as usize] = true;
    let x_max = levels.len() - 1;
    let y_max = levels[0].len() - 1;
    let neighbor: Vec<(i32, i32)> = vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    let valid: Vec<&(i32, i32)> = neighbor
        .iter()
        .filter(|(i, j)| *i >= 0 && *i <= x_max as i32 && *j >= 0 && *j <= y_max as i32)
        .collect();
    for (i, j) in valid.iter() {
        levels[*i as usize][*j as usize] += 1;
        if levels[*i as usize][*j as usize] > 9 {
            increment_neighbor(levels, flash_map, *i as i32, *j as i32);
        }
    }
}

fn next_step(levels: &mut Vec<Vec<i32>>) -> i32 {
    let x_max = levels.len() - 1;
    let y_max = levels[0].len() - 1;
    let mut flash_map: Vec<Vec<bool>> = vec![vec![false; levels[0].len()]; levels.len()];
    let mut count: i32 = 0;
    for i in 0..=x_max {
        for j in 0..=y_max {
            levels[i][j] += 1;
            if levels[i][j] > 9 {
                increment_neighbor(levels, &mut flash_map, i as i32, j as i32);
            }
        }
    }
    for i in 0..=x_max {
        for j in 0..=y_max {
            if levels[i][j] > 9 {
                count += 1;
                levels[i][j] = 0;
            }
        }
    }
    count
}

fn part_1(levels: &Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    let mut levels = levels.clone();
    for _ in 0..100 {
        sum += next_step(&mut levels);
    }
    sum
}

fn part_2(levels: &Vec<Vec<i32>>) -> u32 {
    let mut levels = levels.clone();
    let mut count: u32 = 0;
    loop {
        count += 1;
        if next_step(&mut levels) == (levels.len() * levels[0].len()) as i32 {
            break;
        }
    }
    count
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();

    let f = |x: &str| {
        x.chars()
            .map(|y| y.to_string().parse::<i32>().unwrap())
            .collect()
    };
    let mut levels: Vec<Vec<i32>> = vec![];
    for i in content_lines.iter() {
        let temp = f(i);
        levels.push(temp);
    }

    println!("{}", part_1(&levels));
    println!("{}", part_2(&levels));
}
