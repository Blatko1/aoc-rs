fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut lines: Vec<&str> = input.lines().collect();
    let operations: Vec<&str> =
        lines.pop().unwrap().split_whitespace().collect();

    let rows: Vec<Vec<u64>> = lines
        .iter()
        .map(|line| {
            line.split_whitespace()
                .map(|e| e.parse().unwrap())
                .collect()
        })
        .collect();

    let width = rows[0].len();
    let height = rows.len();

    let mut total = 0;
    for x in 0..width {
        let result = match operations[x] {
            "+" => {
                let mut sum = 0;
                for y in 0..height {
                    sum += rows[y][x];
                }
                sum
            }
            "*" => {
                let mut product = 1;
                for y in 0..height {
                    product *= rows[y][x];
                }
                product
            }
            _ => unreachable!(),
        };
        total += result;
    }

    total
}

fn part2(input: &str) -> u64 {
    let mut rows: Vec<Vec<char>> =
        input.lines().map(|row| row.chars().collect()).collect();
    let operations: Vec<char> = rows.pop().unwrap();

    let height = rows.len();

    let mut total = 0;

    let mut current_operation_idx = 0;
    while current_operation_idx < operations.len() {
        let next = operations
            .iter()
            .skip(current_operation_idx + 1)
            .position(|c| matches!(c, '+' | '*'))
            .unwrap_or(operations.len() - current_operation_idx);
        let from = current_operation_idx;
        let to = current_operation_idx + next;

        let result = match operations[current_operation_idx] {
            '+' => {
                let mut sum = 0;
                for x in from..to {
                    let mut acc = String::new();
                    for y in 0..height {
                        let digit = rows[y][x];
                        acc.push(digit);
                    }
                    //println!("{}", acc);
                    sum += acc.trim().parse::<u64>().unwrap();
                }
                sum
            }
            '*' => {
                let mut product = 1;
                for x in from..to {
                    let mut acc = String::new();
                    for y in 0..height {
                        let digit = rows[y][x];
                        acc.push(digit);
                    }
                    //println!("{}", acc);
                    product *= acc.trim().parse::<u64>().unwrap();
                }
                product
            }
            _ => unreachable!(),
        };
        //println!("res_ {}", result);

        total += result;

        current_operation_idx += next + 1;
    }

    total
}
