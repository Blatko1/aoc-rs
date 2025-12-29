fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result:{}", part1(input));
    println!("Part 2 result:{}", part2(input));
}

fn part1(input: &str) -> i32 {
    let mut dial = 50;
    let mut pass = 0;
    for line in input.lines() {
        let line = line.trim();
        let num = line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => {
                // Or just rem-euclid
                dial -= num;
                dial %= 100;
                if dial == 0 {
                    pass += 1;
                } else if dial < 0 {
                    dial = 100 + dial;
                }
            }
            "R" => {
                dial += num;
                dial %= 100;
                if dial == 0 {
                    pass += 1;
                }
            }
            _ => unreachable!(),
        }
        println!("dial: {dial}");
    }
    pass
}

fn part2(input: &str) -> i32 {
    let mut dial = 50;
    let mut pass = 0;
    for line in input.lines() {
        let line = line.trim();
        let num = line[1..].parse::<i32>().unwrap();
        match &line[0..1] {
            "L" => {
                let off = if dial == 0 { 0 } else { 1 };
                dial -= num;
                if dial > 0 {
                    continue;
                }
                let n = off + (-dial / 100);
                pass += n;

                dial %= 100;
                if dial < 0 {
                    dial = 100 + dial;
                }
            }
            "R" => {
                dial += num;
                let n = dial / 100;
                dial %= 100;
                pass += n;
            }
            _ => unreachable!(),
        }
    }
    pass
}
