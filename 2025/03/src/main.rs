fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();
        let len = line.len();
        let mut chars = line.chars().enumerate().take(len - 1);
        let (mut idx, mut max) = chars.next().unwrap();
        chars.for_each(|(i, c)| {
            if c > max {
                max = c;
                idx = i;
            }
        });
        let mut chars2 = line.chars().skip(1 + idx);
        let mut max2 = chars2.next().unwrap();
        chars2.for_each(|c| {
            if c > max2 {
                max2 = c;
            }
        });
        println!("max: {max}, max2: {max2}");
        let num = max.to_digit(10).unwrap() * 10 + max2.to_digit(10).unwrap();
        sum += num;
    }
    sum
}

fn find_max(line: &str, start: usize, len: usize) -> (usize, char) {
    let l = line.len();
    let mut chars = line.chars().enumerate().skip(start).take(l - len - start);
    let (mut idx, mut max) = chars.next().unwrap();
    chars.for_each(|(i, c)| {
        if c > max {
            max = c;
            idx = i;
        }
    });
    (idx, max)
}

fn part2(input: &str) -> u64 {
    let mut sum = 0;
    for line in input.lines() {
        let line = line.trim();

        let mut start = 0;
        for len in (0..12).rev() {
            let (idx, max) = find_max(line, start, len);
            start = idx + 1;
            print!("{max}");
            sum += 10u64.pow(len as u32) * max.to_digit(10).unwrap() as u64;
        }
        println!();
    }
    sum
}
