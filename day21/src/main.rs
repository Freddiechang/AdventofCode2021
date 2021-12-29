use std::fs;

fn parse_input(content_lines: &Vec<&str>) -> (u32, u32) {
    let f = |x: &str| {
        x.split(": ")
            .skip(1)
            .next()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    };
    return (f(content_lines[0]), f(content_lines[1]));
}

fn part_1((start_1, start_2): (u32, u32)) {
    let mut score_1: u32 = 0;
    let mut score_2: u32 = 0;
    let mut spot_1: u32 = start_1;
    let mut spot_2: u32 = start_2;
    let mut dice: u32 = 1;
    let mut count: u32 = 0;
    let mut player_1 = true;
    while score_1 < 1000 && score_2 < 1000 {
        for _ in 0..3 {
            if player_1 {
                spot_1 = dice % 10 + spot_1;
                if spot_1 > 10 {
                    spot_1 -= 10;
                }
            } else {
                spot_2 = dice % 10 + spot_2;
                if spot_2 > 10 {
                    spot_2 -= 10;
                }
            }
            dice = if dice < 100 { dice + 1 } else { dice - 99 };
        }
        if player_1 {
            score_1 += spot_1;
        } else {
            score_2 += spot_2;
        }
        count += 3;
        player_1 = !player_1;
    }
    println!(
        "{}",
        if player_1 {
            score_1 * count
        } else {
            score_2 * count
        }
    )
}

const ROLL: [u64; 7] = [3, 4, 5, 6, 7, 8, 9];
const FREQ: [u64; 7] = [1, 3, 6, 7, 6, 3, 1];

fn part_2_helper(
    start_1: u32,
    start_2: u32,
    player_1: bool,
    score_1: u32,
    score_2: u32,
) -> (u64, u64) {
    let mut count_1: u64 = 0;
    let mut count_2: u64 = 0;
    let pos = if player_1 { start_1 } else { start_2 };
    let score = if player_1 { score_1 } else { score_2 };
    for (i, item) in ROLL.iter().enumerate() {
        let mut pos_t = pos + *item as u32;
        if pos_t > 10 {
            pos_t -= 10;
        }
        if score + pos_t >= 21 {
            if player_1 {
                count_1 += FREQ[i];
            } else {
                count_2 += FREQ[i];
            }
        } else {
            let mut f = |(x, y)| {
                count_1 += x * FREQ[i];
                count_2 += y * FREQ[i];
            };
            if player_1 {
                f(part_2_helper(pos_t, start_2, false, score + pos_t, score_2));
            } else {
                f(part_2_helper(start_1, pos_t, true, score_1, score + pos_t));
            }
        }
    }
    (count_1, count_2)
}

fn part_2((start_1, start_2): (u32, u32)) {
    println!("{:?}", part_2_helper(start_1, start_2, true, 0, 0));
}

fn main() {
    let filename = String::from("input.txt");
    let contents: String = fs::read_to_string(filename).unwrap();
    let content_lines: Vec<&str> = contents.trim().split('\n').collect();

    part_1(parse_input(&content_lines));
    part_2(parse_input(&content_lines));
}
