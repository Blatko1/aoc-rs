fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    //println!("Part 2 result: {}", part2(input));
}

fn surface(p1: &[i64; 2], p2: &[i64; 2]) -> i64 {
    let width = (p1[0] - p2[0]).abs() + 1;
    let height = (p1[1] - p2[1]).abs() + 1;

    width * height
}

fn part1(input: &str) -> u64 {
    let points: Vec<[i64; 2]> = input
        .lines()
        .map(|line| {
            let mut split = line.split(',');
            [
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    //let pairs: Vec<(f64, usize, usize)> = points.iter().enumerate().flat_map(|(idx1, p1)| points.iter().enumerate().skip(idx1+1).map(move |(idx2, p2)| (dist(p1, p2), idx1, idx2))).collect();
    //let max = pairs.iter().max_by(|a, b| a.0.total_cmp(&b.0)).unwrap();
    //let p1 = points[max.1];
    //let p2 = points[max.2];
    //let size = ((p1[0] as i64 - p2[0] as i64 + 1) * (p1[1] as i64 - p2[1] as i64 + 1)).abs();
    //println!("max: {:?}", size);

    let pairs: Vec<(i64, usize, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(idx1, p1)| {
            points
                .iter()
                .enumerate()
                .skip(idx1 + 1)
                .map(move |(idx2, p2)| (surface(p1, p2), idx1, idx2))
        })
        .collect();
    let max = pairs.iter().max_by_key(|k| k.0).unwrap();
    println!("max: {:?}", max);

    /*let [min_x, _] = points.iter().min_by_key(|k| k[0]).unwrap();
    let [max_x, _] = points.iter().max_by_key(|k| k[0]).unwrap();
    let [_, min_y] = points.iter().min_by_key(|k| k[1]).unwrap();
    let [_, max_y] = points.iter().max_by_key(|k| k[1]).unwrap();

    let most_left = points.iter().filter(|p| p[0] == *min_x);
    let left_bottom = most_left.clone().min_by_key(|k| k[1]).unwrap();
    let left_top = most_left.max_by_key(|k| k[1]).unwrap();

    let most_right = points.iter().filter(|p| p[0] == *max_x);
    let right_bottom = most_right.clone().min_by_key(|k| k[1]).unwrap();
    let rigth_top = most_right.max_by_key(|k| k[1]).unwrap();

    let most_top = points.iter().filter(|p| p[1] == *max_y);
    let top_left = most_top.clone().min_by_key(|k| k[0]).unwrap();
    let top_right = most_top.max_by_key(|k| k[0]).unwrap();

    let most_bottom = points.iter().filter(|p| p[1] == *min_y);
    let bottom_left = most_bottom.clone().min_by_key(|k| k[0]).unwrap();
    let bottom_right = most_bottom.max_by_key(|k| k[0]).unwrap();

    let extremes = vec![left_bottom, left_top, right_bottom, rigth_top, top_left, top_right, bottom_left, bottom_right];
    println!("extremes: {:?}", extremes);
    let mut max_size = 0;
    let mut pair = None;
    for (i, p1) in extremes.iter().enumerate() {
        for p2 in extremes.iter().skip(i + 1) {
            let size = ((p2[1] as i64 - p1[1] as i64 + 1) * (p2[0] as i64 - p1[0] as i64 + 1)).abs() as u64;
            if size > max_size {
                max_size = size;
                pair = Some((p1, p2));
            }
        }
    }
    println!("pair: {:?}", pair);*/

    todo!()
}
