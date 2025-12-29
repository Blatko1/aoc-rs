fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut rolls = 0;
    for y in 0..height {
        for x in 0..width {
            if adjacent_count(&lines, x, y) < 4 && lines[y][x] == '@' {
                rolls += 1;
            }
        }
    }

    rolls
}

fn adjacent_count(lines: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    let width = lines[0].len();
    let height = lines.len();
    let mut adjacent = 0;
    // TL
    if x > 0 && y > 0 {
        if lines[y - 1][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // T
    if y > 0 {
        if lines[y - 1][x] == '@' {
            adjacent += 1;
        }
    }
    // TR
    if x < width - 1 && y > 0 {
        if lines[y - 1][x + 1] == '@' {
            adjacent += 1;
        }
    }

    // L
    if x > 0 {
        if lines[y][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // R
    if x < width - 1 {
        if lines[y][x + 1] == '@' {
            adjacent += 1;
        }
    }

    // BL
    if x > 0 && y < height - 1 {
        if lines[y + 1][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // B
    if y < height - 1 {
        if lines[y + 1][x] == '@' {
            adjacent += 1;
        }
    }
    // BR
    if x < width - 1 && y < height - 1 {
        if lines[y + 1][x + 1] == '@' {
            adjacent += 1;
        }
    }

    adjacent
}

fn part2(input: &str) -> u32 {
    let mut lines: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();
    let width = lines[0].len();
    let height = lines.len();

    let mut rolls = 0;
    let mut last = u32::MAX;
    while last != rolls {
        last = rolls;
        for y in 0..height {
            for x in 0..width {
                if check_and_modify(&mut lines, x, y) {
                    rolls += 1;
                }
            }
        }
    }

    rolls
}

fn check_and_modify(lines: &mut Vec<Vec<char>>, x: usize, y: usize) -> bool {
    if lines[y][x] != '@' {
        return false;
    }
    let width = lines[0].len();
    let height = lines.len();
    let mut adjacent = 0;
    // TL
    if x > 0 && y > 0 {
        if lines[y - 1][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // T
    if y > 0 {
        if lines[y - 1][x] == '@' {
            adjacent += 1;
        }
    }
    // TR
    if x < width - 1 && y > 0 {
        if lines[y - 1][x + 1] == '@' {
            adjacent += 1;
        }
    }

    // L
    if x > 0 {
        if lines[y][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // R
    if x < width - 1 {
        if lines[y][x + 1] == '@' {
            adjacent += 1;
        }
    }

    // BL
    if x > 0 && y < height - 1 {
        if lines[y + 1][x - 1] == '@' {
            adjacent += 1;
        }
    }
    // B
    if y < height - 1 {
        if lines[y + 1][x] == '@' {
            adjacent += 1;
        }
    }
    // BR
    if x < width - 1 && y < height - 1 {
        if lines[y + 1][x + 1] == '@' {
            adjacent += 1;
        }
    }

    if adjacent < 4 {
        lines[y][x] = '.';
        return true;
    }
    false
}
