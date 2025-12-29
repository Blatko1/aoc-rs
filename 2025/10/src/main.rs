use good_lp::{ProblemVariables, variable};
use nalgebra::{Const, DMatrix, DVector, Dyn, SVD, SVector, VecStorage, VectorN};

fn main() {
    let input = include_str!("../input.txt");
    println!("Part 1 result: {}", part1(input));
    println!("Part 2 result: {}", part2(input));
}

//(0  ,1  ,1  ,1  ,0  ,1  )
//a*(x0 + x1 + x2 + x3 + x4) + b*(x0 + x3 + x4) + c*(x0 + x1 + x2 + x4 + x5) + d*(x1 + x2) = (mx0,nx1,ox2,px3,rx4,sx5)
//x0*(a + b + c) = mx0 0
//x1*(a + c + d) = nx1 1
//x2*(a + c + d) = ox1 1
//x3*(a + b)     = px1 1
//x4*(a + b + c) = rx0 0 
//x5*(c)         = sx1 1

fn light_indicator_to_number(c: char) -> i64 {
    if c == '.' {
        0
    } else {
        1
    }
}

fn part1(input: &str) -> i64 {
    let lines = input.lines().map(|line| {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let len = parts.len();
        let target_str = &parts[0];
        let target_len = target_str.len();
        let target: Vec<i64> = target_str[1..target_len-1].chars().map(light_indicator_to_number).collect();
        let buttons: Vec<Vec<i64>> = parts[1..(len-1)].iter().map(|button| {
            let len = button.len();
            button[1..(len-1)].split(',').map(|v| v.parse().unwrap()).collect()
        }).collect();

        (target, buttons)
    });

    let mut total_presses = 0;
    for (target, buttons) in lines {
        let mut press_count: i64 = 1;

        'outer: loop {
            let mut button_presses = vec![0; buttons.len()];
            let mut combination = press_count;
            loop {
                let base = press_count + 1;
                let mut last_non_zero_idx = button_presses.len()-1;
                let mut last_non_zero = 0;
                let mut sum = 0;
                for (i, press) in button_presses.iter_mut().rev().enumerate().rev() {
                    *press = combination / base.pow(i as u32) % base;
                    sum += *press;
                    if *press != 0 {
                        last_non_zero_idx = i;
                        last_non_zero = *press;
                    }
                }
                
                if sum == press_count {
                    let mut result = vec![0; target.len()];
                    for (i, press_count) in button_presses.iter().enumerate() {
                        for output in &buttons[i] {
                            result[*output as usize] += press_count;
                        }
                    }
                    result.iter_mut().for_each(|v| *v = *v % 2);
                    if result == target {
                        println!("buttons: {:?}", button_presses);
                        break 'outer;
                    }
                    combination += (base - last_non_zero) * base.pow(last_non_zero_idx as u32);
                } else {
                    combination += press_count - sum;
                }
                //println!("combination: {:?}, i: {}, l: {}", button_presses, last_non_zero_idx, last_non_zero);
                if combination >= base.pow(button_presses.len() as u32) {
                    break;
                }
            }
            press_count += 1;
        }
        println!("cound: {}", press_count);

        total_presses += press_count;        
    }

    total_presses
}

//(0  ,1  ,1  ,1  ,0  ,1  )
//a*(x0 + x1 + x2 + x3 + x4) + b*(x0 + x3 + x4) + c*(x0 + x1 + x2 + x4 + x5) + d*(x1 + x2) = 10x0 + 11x1 + 11x2 + 5x3 +10x4 +5x5
//x0*(a + b + c) = 10*x0
//x1*(a + c + d) = 11*x1
//x2*(a + c + d) = 11*x2
//x3*(a + b)     = 5*x3
//x4*(a + b + c) = 10*x4
//x5*(c)         = 5*x5

fn part2(input: &str) -> i64 {
    let lines = input.lines();

    let mut total_presses = 0;
    for line in lines {
        let parts: Vec<&str> = line.split_whitespace().collect();
        let len = parts.len();
        let target_str = &parts[len-1];
        let target_len = target_str.len();
        let target: Vec<i64> = target_str[1..target_len-1].split(',').map(|c| c.parse().unwrap()).collect();
        let output_len = target.len();

        let vector = vec![0.0; output_len];
        let buttons: Vec<Vec<f64>> = parts[1..(len-1)].iter().map(|button| {
            let mut vec = vector.clone();
            let len = button.len();
            for output in button[1..(len-1)].split(',') {
                let val: usize = output.parse().unwrap();
                println!("val: {}", val);
                vec[val] = 1.0;
            }
            vec
        }).collect();

        //let mut problem = ProblemVariables::new();
        //for button in &buttons {
        //    problem.add(variable().integer().min(0))
        //}

        /*let columns = buttons.len();
        let rows = buttons[0].len();
        let data: Vec<f64> = buttons.iter().flatten().map(|v| *v as f64).collect();
        
        //for i in 0..(rows - columns) {
        //    data.append(&mut vector.clone());
        //}
        let storage = VecStorage::new(Dyn(rows), Dyn(columns), data);
        let mut matrix = DMatrix::from_data(storage);
        println!("matrix: {}", matrix);

        /*if columns < rows {
            let add = rows - columns;
            println!("cols {columns}");
            matrix = matrix.insert_columns(columns, add, 0.0);
        } else {
            let add = columns - rows;
            println!("rows {rows}");
            matrix = matrix.insert_rows(rows, add, 0.0);
        }
        println!("matrix: {}", matrix);*/
        let inv = matrix.clone().pseudo_inverse(1e-10).unwrap();
        println!("inv: {}", inv);

            // Compute SVD to find null space
        let svd = matrix.clone().transpose().svd(true, true);
        println!("svd: {:?}", svd);
        
        // The null space vectors are the right singular vectors
        // corresponding to near-zero singular values
        let tolerance = 1e-10;
        let v = svd.v_t.unwrap().transpose();

        let target_data: Vec<f64> = target.iter().map(|v| *v as f64).collect();
        let target_storage = VecStorage::new(Dyn(target_data.len()), Dyn(1), target_data);
        let target_vec = DMatrix::from_data(target_storage);

        println!("target: {}", target_vec);
        let result = inv * target_vec;
        println!("result: {}", result);

        //let sum: f64 = result.iter().sum();
        //println!("sum: {}", sum);*/
        panic!()
    }

    total_presses
}
