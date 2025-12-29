fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn part1(input: &str) -> u64 {
    let mut invalids = 0;
    for range in input.split(',') {
        let range = range.split('-').collect::<Vec<&str>>();
        let [left, right] = range.as_slice() else {
            panic!()
        };
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        for num in left..=right {
            let str = num.to_string();
            let len = str.len();
            if str[0..(len / 2)].eq(&str[(len / 2)..]) {
                invalids += num;
            }
        }
    }
    invalids
}

fn contains_pattern(num: u64) -> bool {
    let str = num.to_string();
    let str = str.as_str();
    //println!("str: {str}");
    let len = str.len();
    'outer: for n in 1..=len / 2 {
        if len % n != 0 {
            continue;
        }
        let slice = &str[0..n];
        for idx in 1..(len / n) {
            //println!("idx: {idx}, n: {n}");
            if &str[idx * n..(idx * n + n)] != slice {
                continue 'outer;
            }
        }
        return true;
    }
    return false;
}

fn part2(input: &str) -> u64 {
    let mut invalids = 0;
    for range in input.split(',') {
        let range = range.split('-').collect::<Vec<&str>>();
        let [left, right] = range.as_slice() else {
            panic!()
        };
        let left: u64 = left.parse().unwrap();
        let right: u64 = right.parse().unwrap();
        for num in left..=right {
            if contains_pattern(num) {
                invalids += num;
            }
        }
    }
    invalids
}
