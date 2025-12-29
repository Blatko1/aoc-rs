use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn dist(p1: &[f64; 3], p2: &[f64; 3]) -> f64 {
    let x = p1[0] - p2[0];
    let y = p1[1] - p2[1];
    let z = p1[2] - p2[2];

    (x * x + y * y + z * z).sqrt()
}

fn part1(input: &str) -> usize {
    let points: Vec<[f64; 3]> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            [
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let mut dists: Vec<(f64, usize, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(idx1, p1)| {
            points
                .iter()
                .enumerate()
                .skip(idx1 + 1)
                .map(move |(idx2, p2)| (dist(p1, p2), idx1, idx2))
        })
        .collect();

    dists.sort_by(|a, b| a.0.total_cmp(&b.0));

    let pairs_of_closest: Vec<(usize, usize)> = dists[0..1000]
        .into_iter()
        .map(|(_, idx1, idx2)| (*idx1, *idx2))
        .collect();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for (idx1, idx2) in pairs_of_closest.into_iter() {
        let c1_idx = circuits.iter().position(|set| set.contains(&idx1));
        let c2_idx = circuits.iter().position(|set| set.contains(&idx2));
        match (c1_idx, c2_idx) {
            (None, None) => {
                let mut set = HashSet::new();
                set.insert(idx1);
                set.insert(idx2);
                circuits.push(set);
            }
            (None, Some(set_idx)) => {
                circuits[set_idx].insert(idx1);
            }
            (Some(set_idx), None) => {
                circuits[set_idx].insert(idx2);
            }
            (Some(set1_idx), Some(set2_idx)) => {
                if set1_idx != set2_idx {
                    let set1 = &circuits[set1_idx];
                    let set2 = &circuits[set2_idx];
                    let set: HashSet<_> = set1.union(set2).cloned().collect();
                    let set1 = &mut circuits[set1_idx];
                    *set1 = set;
                    circuits.remove(set2_idx);
                }
            }
        }
    }
    circuits.sort_by(|a, b| b.len().cmp(&a.len()));

    let result = circuits[0].len() * circuits[1].len() * circuits[2].len();
    result
}

fn part2(input: &str) -> u64 {
    let points: Vec<[f64; 3]> = input
        .lines()
        .map(|line| {
            let mut coords = line.split(',');
            [
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
                coords.next().unwrap().parse().unwrap(),
            ]
        })
        .collect();

    let mut dists: Vec<(f64, usize, usize)> = points
        .iter()
        .enumerate()
        .flat_map(|(idx1, p1)| {
            points
                .iter()
                .enumerate()
                .skip(idx1 + 1)
                .map(move |(idx2, p2)| (dist(p1, p2), idx1, idx2))
        })
        .collect();

    dists.sort_by(|a, b| a.0.total_cmp(&b.0));

    let pairs: Vec<(usize, usize)> = dists
        .into_iter()
        .map(|(_, idx1, idx2)| (idx1, idx2))
        .collect();

    let mut circuits: Vec<HashSet<usize>> = Vec::new();
    for (idx1, idx2) in pairs.into_iter() {
        let c1_idx = circuits.iter().position(|set| set.contains(&idx1));
        let c2_idx = circuits.iter().position(|set| set.contains(&idx2));
        match (c1_idx, c2_idx) {
            (None, None) => {
                let mut set = HashSet::new();
                set.insert(idx1);
                set.insert(idx2);
                circuits.push(set);
            }
            (None, Some(set_idx)) => {
                circuits[set_idx].insert(idx1);
            }
            (Some(set_idx), None) => {
                circuits[set_idx].insert(idx2);
            }
            (Some(set1_idx), Some(set2_idx)) => {
                if set1_idx != set2_idx {
                    let set1 = &circuits[set1_idx];
                    let set2 = &circuits[set2_idx];
                    let set: HashSet<_> = set1.union(set2).cloned().collect();
                    let set1 = &mut circuits[set1_idx];
                    *set1 = set;
                    circuits.remove(set2_idx);
                }
            }
        }

        if circuits.len() == 1 && circuits[0].len() == points.len() {
            let p1 = points[idx1];
            let p2 = points[idx2];
            return p1[0] as u64 * p2[0] as u64;
        }
    }
    panic!()
}
