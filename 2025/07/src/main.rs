use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let idx = map[0].iter().position(|c| *c == 'S').unwrap();
    map[1][idx] = '|';

    let mut splits = 0;
    for y in 2..map.len() {
        let (left, right) = map.split_at_mut(y);
        let prev = left.last_mut().unwrap();
        let current = &mut right[0];
        prev.iter()
            .enumerate()
            .filter(|(_, c)| **c == '|')
            .for_each(|(idx, _)| {
                if current[idx] != '^' {
                    current[idx] = '|';
                } else {
                    splits += 1;
                    if current[idx - 1] == '.' {
                        current[idx - 1] = '|'
                    }
                    if current[idx + 1] == '.' {
                        current[idx + 1] = '|'
                    }
                }
            });
    }

    let flat: String = map
        .iter()
        .flat_map(|row| {
            let mut row = row.clone();
            row.push('\n');
            row
        })
        .collect();

    std::fs::write("out.txt", flat).unwrap();

    splits
}

// (x, y) - coordinates of the particle being processed
fn recurse(
    y: usize,
    x: usize,
    map: &[Vec<char>],
    memory: &mut HashMap<(usize, usize), u64>,
) -> u64 {
    if y >= map.len() - 1 {
        return 1;
    }

    if let Some(val) = memory.get(&(x, y + 1)) {
        return *val;
    }
    if map[y + 1][x] == '.' {
        let val = recurse(y + 1, x, map, memory);
        memory.insert((x, y + 1), val);
        return val;
    }

    let val1 = if let Some(val) = memory.get(&(x - 1, y + 1)) {
        *val
    } else {
        let val = recurse(y + 1, x - 1, map, memory);
        memory.insert((x - 1, y + 1), val);
        val
    };
    let val2 = if let Some(val) = memory.get(&(x + 1, y + 1)) {
        *val
    } else {
        let val = recurse(y + 1, x + 1, map, memory);
        memory.insert((x + 1, y + 1), val);
        val
    };

    return val1 + val2;
}

fn part2(input: &str) -> u64 {
    let map: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    let idx = map[0].iter().position(|c| *c == 'S').unwrap();
    let result = recurse(0, idx, &map, &mut HashMap::new());

    result
}
