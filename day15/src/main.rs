use std::fs;

fn update_neighbor(
    map: &Vec<Vec<u32>>,
    visited: &Vec<Vec<bool>>,
    cost: &mut Vec<Vec<u32>>,
    x: usize,
    y: usize,
) {
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;
    let f = |x| x as i32;
    let neighbor: Vec<(i32, i32)> = vec![
        (f(x), f(y) - 1),
        (f(x) - 1, f(y)),
        (f(x) + 1, f(y)),
        (f(x), f(y) + 1),
    ];
    let valid = neighbor.iter().filter(|(i, j)| {
        *i >= 0
            && *i <= x_max as i32
            && *j >= 0
            && *j <= y_max as i32
            && visited[*i as usize][*j as usize] == true
    });
    for i in valid {
        let temp = cost[x][y] + map[i.0 as usize][i.1 as usize];
        if temp < cost[i.0 as usize][i.1 as usize] {
            cost[i.0 as usize][i.1 as usize] = temp;
            update_neighbor(map, visited, cost, i.0 as usize, i.1 as usize);
        }
    }
}

fn shortest_path(map: &Vec<Vec<u32>>) -> u32 {
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut cost: Vec<Vec<u32>> = vec![vec![u16::MAX as u32; map[0].len()]; map.len()];
    cost[0][0] = 0;
    let x_max = map.len() - 1;
    let y_max = map[0].len() - 1;
    let f = |x: usize| x as i32;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let neighbor: Vec<(i32, i32)> = vec![
                (f(x), f(y) - 1),
                (f(x) - 1, f(y)),
                (f(x) + 1, f(y)),
                (f(x), f(y) + 1),
            ];
            let valid = neighbor
                .iter()
                .filter(|(i, j)| *i >= 0 && *i <= x_max as i32 && *j >= 0 && *j <= y_max as i32);
            for k in valid {
                let temp = cost[k.0 as usize][k.1 as usize] + map[x][y];
                if temp < cost[x][y] {
                    cost[x][y] = temp;
                }
            }
            visited[x][y] = true;
            update_neighbor(map, &visited, &mut cost, x, y);
        }
    }
    cost[cost.len() - 1][cost[0].len() - 1]
}

fn part_1(content_lines: &Vec<&str>) -> u32 {
    let map: Vec<Vec<u32>> = content_lines
        .iter()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    shortest_path(&map)
}

fn part_2(content_lines: &Vec<&str>) -> u32 {
    let map: Vec<Vec<u32>> = content_lines
        .iter()
        .map(|x| x.chars().map(|x| x.to_digit(10).unwrap()).collect())
        .collect();
    let mut new_map: Vec<Vec<u32>> = vec![vec![]; map.len() * 5];
    let f = |x: &Vec<u32>, i: u32| -> Vec<u32> {
        x.iter().map(|x| (x + i) % 10 + (x + i) / 10).collect()
    };
    for i in 0..5 {
        for j in 0..5 {
            for k in 0..map.len() {
                let temp = f(&map[k], i + j);
                new_map[i as usize * map.len() + k].extend(temp);
            }
        }
    }
    shortest_path(&new_map)
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();
    println!("{}", part_1(&content_lines));
    println!("{}", part_2(&content_lines));
}
