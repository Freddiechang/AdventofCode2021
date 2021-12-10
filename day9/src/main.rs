use std::fs;

fn part_1(heights: &Vec<Vec<i32>>) -> i32 {
    let mut sum: i32 = 0;
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            if i != 0 { if heights[i][j] >= heights[i - 1][j] { continue; } }
            if i != heights.len() - 1 { if heights[i][j] >= heights[i + 1][j] { continue; } }
            if j != 0 { if heights[i][j] >= heights[i][j - 1] { continue; } }
            if j != heights[0].len() - 1 { if heights[i][j] >= heights[i][j + 1] { continue; } }
            sum += heights[i][j] + 1;
        }
    }
    sum
}

fn search_basin(heights: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: usize, j: usize) -> i32 {
    let mut sum = 0;
    if !visited[i][j] && heights[i][j] != 9 {
        sum += 1;
        visited[i][j] = true;
        if i != 0 {
            if heights[i][j] < heights[i - 1][j] {
                sum += search_basin(heights, visited, i - 1, j);
            }
        }
        if i != heights.len() - 1 {
            if heights[i][j] < heights[i + 1][j] {
                sum += search_basin(heights, visited, i + 1, j);
            }
        }
        if j != 0 {
            if heights[i][j] < heights[i][j - 1] {
                sum += search_basin(heights, visited, i, j - 1);
            }
        }
        if j != heights[0].len() - 1 {
            if heights[i][j] < heights[i][j + 1] {
                sum += search_basin(heights, visited, i, j + 1);
            }
        }
    }
    sum
}


fn part_2(heights: &Vec<Vec<i32>>) -> i32 {
    let mut biggest: Vec<i32> = vec![0; 3];
    let mut visited: Vec<Vec<bool>> = vec![vec![false; heights[0].len()]; heights.len()];
    for i in 0..heights.len() {
        for j in 0..heights[0].len() {
            if i != 0 { if heights[i][j] >= heights[i - 1][j] { continue; } }
            if i != heights.len() - 1 { if heights[i][j] >= heights[i + 1][j] { continue; } }
            if j != 0 { if heights[i][j] >= heights[i][j - 1] { continue; } }
            if j != heights[0].len() - 1 { if heights[i][j] >= heights[i][j + 1] { continue; } }
            let temp = search_basin(heights, &mut visited, i, j);
            for (idx, k) in biggest.iter().enumerate() {
                if temp >= *k {
                    biggest.insert(idx, temp);
                    biggest.truncate(3);
                    break;
                }
            }
        }
    }
    biggest[0] * biggest[1] * biggest[2]
}



fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();
    let f = |x: &str| x.chars().map(|y| y.to_string().parse::<i32>().unwrap()).collect();
    let mut heights: Vec<Vec<i32>> = vec![];
    for i in content_lines.iter() {
        let temp = f(i);
        heights.push(temp);
    }
    
    println!("{}", part_1(&heights));
    println!("{}", part_2(&heights));
}
