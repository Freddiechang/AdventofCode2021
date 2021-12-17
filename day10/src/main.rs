use std::fs;

fn part_1(content_lines: &Vec<&str>) -> i32 {
    let mut sum: i32 = 0;
    for i in content_lines.iter() {
        let mut s: Vec<char> = vec![];
        for j in i.chars() {
            match j {
                '(' | '[' | '{' | '<' => {
                    s.push(j);
                }
                ')' => {
                    let last = s.pop();
                    match last {
                        Some('(') => {
                            continue;
                        }
                        _ => {
                            sum += 3;
                            break;
                        }
                    }
                }
                ']' => {
                    let last = s.pop();
                    match last {
                        Some('[') => {
                            continue;
                        }
                        _ => {
                            sum += 57;
                            break;
                        }
                    }
                }
                '}' => {
                    let last = s.pop();
                    match last {
                        Some('{') => {
                            continue;
                        }
                        _ => {
                            sum += 1197;
                            break;
                        }
                    }
                }
                '>' => {
                    let last = s.pop();
                    match last {
                        Some('<') => {
                            continue;
                        }
                        _ => {
                            sum += 25137;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
    }
    sum
}

fn part_2(content_lines: &Vec<&str>) -> u64 {
    let mut sum: u64;
    let mut scores: Vec<u64> = vec![];
    for i in content_lines.iter() {
        sum = 0;
        let mut s: Vec<char> = vec![];
        let mut flag = true;
        for j in i.chars() {
            match j {
                '(' | '[' | '{' | '<' => {
                    s.push(j);
                }
                ')' => {
                    let last = s.pop();
                    match last {
                        Some('(') => {
                            continue;
                        }
                        _ => {
                            flag = false;
                            break;
                        }
                    }
                }
                ']' => {
                    let last = s.pop();
                    match last {
                        Some('[') => {
                            continue;
                        }
                        _ => {
                            flag = false;
                            break;
                        }
                    }
                }
                '}' => {
                    let last = s.pop();
                    match last {
                        Some('{') => {
                            continue;
                        }
                        _ => {
                            flag = false;
                            break;
                        }
                    }
                }
                '>' => {
                    let last = s.pop();
                    match last {
                        Some('<') => {
                            continue;
                        }
                        _ => {
                            flag = false;
                            break;
                        }
                    }
                }
                _ => {}
            }
        }
        if flag {
            for i in 0..s.len() {
                sum *= 5;
                match s[s.len() - 1 - i] {
                    '(' => {
                        sum += 1;
                    }
                    '[' => {
                        sum += 2;
                    }
                    '{' => {
                        sum += 3;
                    }
                    '<' => {
                        sum += 4;
                    }
                    _ => {}
                }
            }
            scores.push(sum);
        }
    }
    scores.sort();
    scores[scores.len() / 2]
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.split('\n').collect();

    println!("{}", part_1(&content_lines));
    println!("{}", part_2(&content_lines));
}
