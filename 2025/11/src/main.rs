use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

fn recurse<'a>(map: &HashMap<&str, Vec<&'a str>>, memory: &mut HashMap<&'a str, u64>, key: &str) -> u64 {
    if key == "out" {
        return 1;
    }
    if let Some(path_len) = memory.get(key) {
        return *path_len;
    }
    let paths = &map[key];

    let total_path_len = paths.iter().map(|path| {
        let path_len = recurse(map, memory, path);
        memory.insert(*path, path_len);
        path_len
    }).sum();

    return total_path_len;
}

fn part1(input: &str) -> u64 {
    let map: HashMap<&str, Vec<&str>> = input.lines().map(|line| 
        {
            let mut split = line.split(':');
            let key = split.next().unwrap().trim();
            let values: Vec<&str> = split.next().unwrap().trim().split_whitespace().collect();
            (key, values)
        }
    ).collect();

    let mut memory = HashMap::new();
    let total = recurse(&map, &mut memory, "you");

    total
}

fn recurse2<'a>(map: &HashMap<&str, Vec<&'a str>>, memory: &mut HashMap<(&'a str, bool, bool), u64>, key: &str, mut dac_visited: bool, mut fft_visited: bool) -> u64 {
    if key == "out" {
        if dac_visited && fft_visited {
            return 1;
        }    else {
            return 0;
        }
    }
    
    if key == "dac" {
        dac_visited = true;
    } else if key == "fft" {
        fft_visited = true;
    }
    if let Some(path_len) = memory.get(&(key, dac_visited, fft_visited)) {
        return *path_len;
    }
    let paths = &map[key];

    let total_path_len = paths.iter().map(|path| {
        let path_len = recurse2(map, memory, path, dac_visited, fft_visited);
        memory.insert((*path, dac_visited, fft_visited), path_len);
        path_len
    }).sum();

    return total_path_len;
}

fn part2(input: &str) -> u64 {
    let map: HashMap<&str, Vec<&str>> = input.lines().map(|line| 
        {
            let mut split = line.split(':');
            let key = split.next().unwrap().trim();
            let values: Vec<&str> = split.next().unwrap().trim().split_whitespace().collect();
            (key, values)
        }
    ).collect();

    let mut memory = HashMap::new();
    let total = recurse2(&map, &mut memory, "svr", false, false);

    total
}