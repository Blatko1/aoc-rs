use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let lines: Vec<&str> = input.lines().collect();
    let cut = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap();
    let (fresh_ids_str, available_ids_str) = lines.split_at(cut);
    let available_ids_str = &available_ids_str[1..];
    let fresh_ids: Vec<RangeInclusive<u64>> = fresh_ids_str
        .iter()
        .map(|range_str| {
            let (left, right) = range_str.split_once('-').unwrap();
            let from: u64 = left.parse().unwrap();
            let to: u64 = right.parse().unwrap();
            from..=to
        })
        .collect();
    let available_ids: Vec<u64> = available_ids_str
        .iter()
        .map(|n| n.parse().unwrap())
        .collect();

    let mut fresh_ones = 0;
    for id in available_ids {
        if fresh_ids.iter().any(|range| range.contains(&id)) {
            fresh_ones += 1;
        }
    }

    fresh_ones
}

fn are_overlapping(a: (u64, u64), b: (u64, u64)) -> bool {
    a.1 >= b.0 && a.0 <= b.1
}

fn get_merged(a: (u64, u64), b: (u64, u64)) -> (u64, u64) {
    let from = a.0.min(b.0);
    let to = a.1.max(b.1);
    (from, to)
}

fn part2(input: &str) -> u64 {
    let lines: Vec<&str> = input.lines().collect();
    let cut = lines
        .iter()
        .position(|line| line.trim().is_empty())
        .unwrap();
    let (fresh_ids_str, _) = lines.split_at(cut);

    let mut fresh_ids: Vec<(u64, u64)> = fresh_ids_str
        .iter()
        .map(|range_str| {
            let (left, right) = range_str.split_once('-').unwrap();
            let from: u64 = left.parse().unwrap();
            let to: u64 = right.parse().unwrap();
            (from, to)
        })
        .collect();

    fresh_ids.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total = 0;
    let mut i = 0;

    while i < fresh_ids.len() {
        let (start, mut end) = fresh_ids[i];
        i += 1;

        while i < fresh_ids.len() && fresh_ids[i].0 <= end {
            end = end.max(fresh_ids[i].1);
            i += 1;
        }

        total += end - start + 1;
    }

    /*let mut total = 0;
    // 1-10, 5-15, 3-12
    let mut left_idx = 0;
    loop {
        let start_left = fresh_ids[left_idx];
        let mut left = start_left;
        let mut right_idx = left_idx;

        let mut idx = left_idx+1;
        loop {
            if idx >= fresh_ids.len() {
                break;
            }
            let right = fresh_ids[idx];
            if right.0 <= left.1 {
                right_idx = idx;
                left = right;
                idx += 1;
                continue;
            }
            break;
        }
        if left_idx == right_idx {
            left_idx += 1;
            println!("{} to {}", left.0, left.1);
            total += left.1 - left.0 + 1;
        } else {
            let right = fresh_ids[right_idx];
            println!("{} to {}", start_left.0, right.1);
            total += right.1 - start_left.0 + 1;
            left_idx = right_idx + 1;
        }
        if left_idx >= fresh_ids.len() {
            break;
        }
    }*/
    for range in fresh_ids {
        println!("{:?}", range);
    }
    //println!("ids: {:?}", fresh_ids);

    total
}
